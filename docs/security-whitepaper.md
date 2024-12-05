# Security Whitepaper

**Author**: Lark (they/it)\
**Contact**: <lark@lark.gay>\
**GitHub**: <https://github.com/justlark/notwithout.help>\
**Last Updated**: 5 Dec 2024

This document is an informal overview of how [Not Without
Help](https://notwithout.help) mitigates security risks.

⚠️ This document is in an early draft stage. ⚠️

## Disclaimer

**This system has not been audited for security.** If your safety or freedom
are at risk, do not depend on this software to protect you.

## Overview

Not Without Help is an app that allows users, called **Organizers**, to create
encrypted web forms, collecting **Submissions** from others. **Submissions**
are encrypted client-side such that they cannot be read by the server.

When an **Organizer** creates a **Form**, they are given two links: a **Sharing
Link** that can be followed to fill out the **Form**, and a **Secret Link**
that can be used to view the **Submissions**.

The **Private Primary Key** is generated on the **Organizer's** device, and its
corresponding public key is sent to the server when a **Form** is created. The
**Public Primary Key** is then sent to clients and used to encrypt
**Submissions**.

## Anatomy of a link

A **Sharing Link** has this format:

```
https://notwithout.help/share/#/<form_id>
```

A **Secret Link** has this format:

```
https://notwithout.help/view/#/<form_id>/<key_id>/<key>
```

- `form_id`: The **Form ID**, a unique identifier for the **Form**.
- `key_id`: The **Client Key ID**, a unique identifier for a **Wrapped Private
  Primary Key** (see below).
- `key`: The **Secret Link Key**, used to authenticate with the API and decrypt
  a **Wrapped Private Primary Key** to reveal the **Private Primary Key** (see
  below).

The `key` is stored in the URL fragment rather than the path or query
parameters so it's not leaked to the CDN.

The `form_id` is also stored in the URL fragment so the CDN does not know which
**Form** a user is filling out.

## Creating a form

After creating a **Form**, the **Organizer** is given the **Sharing Link** and
a **Secret Link**. **Organizers** can create additional **Secret Links** as
well. **Secret Links** can have comments attached to them and can be revoked at
any time.

To create a new **Form**:

1. The client generates a random **Private Primary Key** and **Public Primary
   Key**. This will be used by clients to to encrypt **Submissions**.
2. The client generates a random **Secret Link Key** which will form part of
   the **Secret Link**.
3. The client derives a symmetric **Secret Wrapping Key** from the **Secret
   Link Key**.
4. The client derives a **Private Signing Key** and **Public Signing Key** from
   the **Secret Link Key**.
5. The client sends the **Public Primary Key** and **Public Signing Key** to
   the server to create a new **Form**.
6. The server returns a unique **Form ID** and a **Client Key ID** that
   identifies the **Secret Link** in the context of a **Form**.
7. The client uses the **Secret Wrapping Key** to encrypt the **Private Primary
   Key** and generate a **Wrapped Private Primary Key**.
8. The client uses the **Form ID**, **Client Key ID**, and **Secret Signing
   Key** to authenticate with the server, as described in the
   [Authentication](#authentication) section.
9. The client calls an authenticated API endpoint to send the **Wrapped Private
   Primary Key** to the server.
10. The server stores the **Wrapped Private Primary Key** in the database
    alongside its corresponding **Public Signing Key** and **Client Key ID**.
11. The **Form ID**, **Client Key ID**, and **Secret Link Key** form the
    initial **Secret Link**.

## Generating a new secret link

To generate a new **Secret Link**:

1. The client retrieves and decrypts the **Private Primary Key** as described
   in the [Retrieving the private primary
   key](#retrieving-the-private-primary-key) section.
2. The client generates a new random **Secret Link Key**.
3. The client derives a new **Secret Wrapping Key** from the **Secret Link
   Key**.
4. The client derives a new **Private Signing Key** and **Public Signing Key**
   from the **Secret Link Key**.
5. The client uses the new **Secret Wrapping Key** to encrypt the **Private
   Primary Key** and generate a new **Wrapped Private Primary Key**.
6. A user-provided comment for the key is encrypted with the **Secret Wrapping
   Key**.
7. The client sends the new **Wrapped Private Primary Key**, **Public Signing
   Key**, and the encrypted comment to the server via an authenticated
   endpoint.
8. The server generates a new **Client Key ID**. It stores the **Wrapped
   Private Primary Key**, **Secret Wrapping Key**, and encrypted comment in the
   database by the **Client Key ID**.
9. The server returns the new **Client Key ID** to the client.
10. The **Form ID**, new **Client Key ID**, and new **Secret Link Key** form
    the new **Secret Link**.

A **Secret Link** can be revoked by the **Organizer** via an authenticated
endpoint. This deletes the **Wrapped Private Primary Key** and **Public Signing
Key** from the database.

Note that once a **Secret Link** has been used to reveal the **Private Primary
Key**, while revoking it will deny API access, it will not deny the ability to
decrypt **Submissions** if the ciphertext is leaked.

## Retrieving the private primary key

A **Secret Link** can be used to retrieve and decrypt the **Private Primary
Key**.

1. The client uses the **API Access Token** to call an authenticated API
   endpoint (as described in the [Authentication](#authentication) section) to
   get the **Wrapped Private Primary Key**.
2. The client uses the **Secret Wrapping Key** derived from the **Secret Link**
   to decrypt the **Wrapped Private Primary Key** and reveal the **Private
   Primary Key**.

From here, the client can call a different authenticated API endpoint to get
the list of encrypted **Submissions**, which can be decrypted using the
**Private Primary Key**.

## Authentication

Some API endpoints require authentication. See the [API](#api) section for
details.

![Authentication flow diagram](./auth-flow-diagram.png)

The client uses the **Form ID** and **Client Key ID** from the **Secret Link**
to request an **API Challenge** from the server via an unauthenticated API
endpoint.

The server generates a random **Ephemeral Server Key** and stores it in a
key-value store with TTL slightly longer than the `exp` of the **API Access
Token** (see below). This key is assigned a random **Server Key ID**.

The server generates the **API Challenge**, which is a JWT signed with the
**Ephemeral Server Key** via `HS256`. The **API Challenge** has the following
claims:

- `kid` (header claim): The **Server Key ID** of the **Ephemeral Server Key**
  used to sign the JWT.
- `iss` (registered claim): The server's origin.
- `aud` (registered claim): The server's origin.
- `sub` (registered claim): The concatenation of the **Form ID** and the
  **Client Key ID**.
- `iat` (registered claim): The current timestamp.
- `exp` (registered claim): A short expiration timestamp (e.g. 1 minute from
  `iat`).
- `jti` (registered claim): A random nonce generated by the server to ensure
  the **API Challenge** can only be used once. This is stored in a key-value
  store with a TTL slightly longer than the `exp`.
- `type` (custom claim): The string `challenge`, used to distinguish the **API
  Challenge** from an **API Access Token**.
- `nonce` (custom claim): A random nonce generated by the server.

The server returns the **API Challenge** to the client, which decodes (but does
not verify) it to get the `nonce`.

The client signs the nonce with the **Private Signing Key** and generates an
**API Challenge Response** from that signature and the **API Challenge**. The
**API Challenge Response** is a JSON object with the following format:

```json
{
  "signature": "<nonce_signature>",
  "challenge": "<api_challenge>"
}
```

The **API Challenge Response** is then sent back to the server via an
unauthenticated API endpoint to exchange it for an **API Access Token**.

The server verifies:

- The signature of the **API Challenge** using the **Ephemeral Server Key**
  associated with the `kid` to ensure the nonce the client signed is the same
  one the server issued.
- The `type` is `challenge`.
- The `iss` matches the server's origin.
- The `aud` matches the server's origin.
- The `exp` shows the **API Challenge** has not expired.
- The `jti` is still in the key-value store, to ensure the **API Challenge**
  can only be used once.
- The signature of the signed nonce using the **Public Signing Key** associated
  with the **Form ID** and **Client Key ID**.

The server then deletes the `jti` from the key-value store.

The server exchanges the **API Challenge Response** for an **API Access
Token**. The **API Access Token** is a JWT signed with the **Ephemeral Server
Key** via `HS256`. The **API Access Token** has the following claims:

- `kid` (header claim): The **Server Key ID** of the **Ephemeral Server Key**
  used to sign the JWT.
- `iss` (registered claim): The server's origin.
- `aud` (registered claim): The server's origin.
- `sub` (registered claim): The concatenation of the **Form ID** and the
  **Client Key ID**.
- `iat` (registered claim): The current timestamp.
- `exp` (registered claim): The time the **API Access Token** expires, which is
  configurable.
- `type` (custom claim): The string `access`, used to distinguish the **API
  Access Token** from an **API Challenge**.

On subsequent authenticated API requests, the client includes the **API Access
Token** as a bearer token in the `Authorization` header, which the server
validates to authorize the request. The server validates:

- The signature of the **API Access Token** using the **Ephemeral Server Key**
  associated with the `kid`.
- The `type` is `access`.
- The `iss` matches the server's origin.
- The `aud` matches the server's origin.
- The `exp` shows the token has not expired.
- The **Form ID** in the `sub` matches the resource being requested.
- The **Client Key ID** in the `sub` has not been revoked.

## Algorithms

- **Submissions** are encrypted with the **Public Primary Key** using
  [libsodium](https://doc.libsodium.org/public-key_cryptography/sealed_boxes)
  via `crypto_box_seal`.
- The **Private Primary Key** is encrypted with the **Secret Wrapping Key**
  using
  [libsodium](https://doc.libsodium.org/secret-key_cryptography/secretbox) via
  `crypto_secretbox_easy`.
- Comments on **Secret Links** are encrypted with the **Secret Wrapping Key**
  using
  [libsodium](https://doc.libsodium.org/secret-key_cryptography/secretbox) via
  `crypto_secretbox_easy`.
- **API Challenges** and **API Access Tokens** are signed with the **Ephemeral
  Server Key** using [jsonwebtoken](https://crates.io/crates/jsonwebtoken) via
  `HS256` (HMAC-SHA256).
- The **API Challenge** nonce is signed with the **Private Signing Key** using
  [noble-ed25519](https://www.npmjs.com/package/@noble/ed25519) via `sign`.
- The **Public Primary Key** and **Private Primary Key** are generated using
  [libsodium](https://doc.libsodium.org/public-key_cryptography/sealed_boxes)
  via `crypto_box_keypair`.
- The **Secret Link Key** is generated using
  [libsodium](https://doc.libsodium.org/key_derivation) via
  `crypto_kdf_keygen`.
- The **Secret Wrapping Key** is derived from the **Secret Link Key** using
  [libsodium](https://doc.libsodium.org/key_derivation) via
  `crypto_kdf_derive_from_key`.
- The **Private Signing Key** is derived from the **Secret Link Key** using
  [libsodium](https://doc.libsodium.org/key_derivation) via
  `crypto_kdf_derive_from_key`.
- The **Public Signing Key** is derived from the **Private Signing Key** using
  [noble-ed25519](https://www.npmjs.com/package/@noble/ed25519) via
  `getPublicKey`.
- The **Ephemeral Server Key** is 32 random bytes generated by a CSPRNG.

## API

This section lists the authenticated and unauthenticated API endpoints exposed
by the server.

### Authenticated endpoints

Request the ciphertext of the encrypted **Submissions** for a **Form**.

```
GET /submissions/:form_id
```

Delete the **Form** from the database, along with all its associated
**Submissions**, **Wrapped Private Primary Keys**, and **Public Signing Keys**.

```
DELETE /forms/:form_id
```

Get a **Wrapped Private Primary Key** by its **Client Key ID**.

```
GET /keys/:form_id/:client_key_id
```

Send a **Wrapped Private Primary Key**, **Public Signing Key**, and encrypted
comment to the server, associated with a **Form**.

```
POST /keys/:form_id
```

Update the **Wrapped Private Primary Key** and/or encrypted comment associated
with a **Client Key ID**.

```
PATCH /keys/:form_id/:client_key_id
```

List the **Client Key IDs** associated with a **Form**, along with their
respective encrypted comments.

```
GET /keys/:form_id
```

Revoke a secret link by deleting its associated **Wrapped Private Primary Key**
and **Public Signing Key**.

```
DELETE /keys/:form_id/:client_key_id
```

### Unauthenticated endpoints

Create a new **Form**.

```
POST /forms
```

Get a **Form** by its **Form ID**.

```
GET /forms/:form_id
```

Send an encrypted **Submission**.

```
POST /submissions/:form_id
```

Request an **API Challenge**.

```
GET /challenges/:form_id/:client_key_id
```

Exchange an **API Challenge Response** for an **API Access Token**.

```
POST /tokens
```

## Glossary

- **Form**: A web form for collecting **Submissions** from users.
- **Submission**: Information encrypted locally with the **Public Primary Key**
  and sent to the server.
- **Organizer**(s): The user who creates a **Form** and has access to its
  **Secret Link**(s).
- **Sharing Link**: A URL that can be followed to fill out a **Form**.
- **Secret Link**: A URL that can be used to decrypt and view **Submissions**.
- **Private Primary Key**: A private key generated by the **Organizer** that is
  used to decrypt **Submissions**.
- **Public Primary Key**: A public key generated by the **Organizer** that is
  used to encrypt **Submissions**.
- **Ephemeral Server Key**: An ephemeral symmetric key generated by the server
  that is used to sign the **API Challenge** and **API Access Token** for a
  given session.
- **API Challenge**: A JWT which forms part of the flow for authenticating a
  client with the server.
- **API Challenge Response**: A client's response to an **API Challenge**,
  which can be exchanged for an **API Access Token**.
- **API Access Token**: A JWT which is used to authenticate API requests.
- **Secret Wrapping Key**: A symmetric key derived from the **Secret Link Key**
  that is used to encrypt the **Private Primary Key**, generating a **Wrapped
  Private Primary Key**.
- **Wrapped Private Primary Key**: The **Private Primary Key** encrypted with a
  **Secret Wrapping Key**.
- **Private Signing Key**: A private key derived from the **Secret Link Key**
  that is used to sign the nonce in an **API Challenge** to form an **API
  Challenge Response**.
- **Public Signing Key**: The public key corresponding to the **Private Signing
  Key** that is used to verify the signature of the nonce in an **API Challenge
  Response**.
- **Secret Link Key**: A symmetric key that forms part of a **Secret Link** and
  is used to authenticate with the API and decrypt a **Wrapped Private Primary
  Key**.
- **Form ID**: A unique non-secret identifier for a form.
- **Client Key ID**: A non-secret identifier for a **Secret Link** that is
  unique within the context of a **Form**.
- **Server Key ID**: A unique, non-secret identifier for a **Ephemeral Server
  Key**.
