# Security Whitepaper

**Author**: Lark (they/it)\
**Contact**: <lark@lark.gay>\
**GitHub**: <https://github.com/justlark/notwithout.help>

This document is an informal overview of how [Not Without
Help](https://notwithout.help) mitigates security risks.

## Disclaimer

**This system has not been audited for security.** If your safety or freedom
are at risk, do not depend on this software to protect you.

## Overview

Not Without Help is an app that allows users, called **Organizers** to create
encrypted web forms, collecting **Submissions** from others. **Submissions**
are encrypted client-side such that they cannot be read by the server.

When an **Organizer** creates a **Form**, they are given two links: a **Sharing
Link** that can be followed to fill out the **Form**, and a **Secret Link**
that can be used to view the **Submissions**.

The **Private Client Key** is generated on the **Organizer's** device, and its
corresponding public key is sent to the server when a **Form** is created. The
**Public Client Key** is then sent to clients and used to encrypt
**Submissions** via a libsodium [**Sealed
Box**](https://doc.libsodium.org/public-key_cryptography/sealed_boxes).

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
  Client Key** (see below).
- `key`: The **Private Wrapping Key**, used to authenticate with the API and
  decrypt a **Wrapped Private Client Key** to reveal the **Private Client Key**
  (see below).

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

1. The client generates two random key pairs: The **Private Client Key**, the
   **Public Client Key**, the **Private Wrapping Key** and the **Public
   Wrapping Key**.
2. The client sends the **Public Client Key** and the **Public Wrapping Key**
   to the server to create a new **Form**.
3. The server generates a random key pair for the **Form** called the **Private
   Server Key** and **Public Server Key**, along with a unique **Server Key
   ID**. The purpose of the **Server Key ID** is explained in the [Key
   rotation](#key-rotation) section.
4. The server returns the **Public Server Key**, **Server Key ID**, a unique
   **Form ID**, and a **Client Key ID** for the initial **Secret Link** to the
   client.
5. The client uses the **Public Wrapping Key** to encrypt the **Private Client
   Key** via a **Sealed Box** to generate a **Wrapped Private Client Key** that
   only the **Private Wrapping Key** can decrypt.
6. The client uses the **Form ID**, **Client Key ID**, **Server Key ID**,
   **Private Wrapping Key**, and **Public Server Key** to call an authenticated
   API endpoint (as described in the [Authentication](#authentication) section)
   to send the **Wrapped Private Client Key** to the server.
7. The server stores the **Wrapped Private Client Key** in the database
   alongside its corresponding **Public Wrapping Key** and **Client Key ID**.
8. The **Form ID**, **Client Key ID**, and **Private Wrapping Key** form the
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

The client uses the **Form ID** and **Client Key ID** from the **Secret Link**
to request an **API Challenge** from the server via an unauthenticated API
endpoint.

The server generates the **API Challenge** which is a JWT signed with the
**Secret Server Key** via `HS256`. The **API Challenge** has the following
claims:

- `kid` (header claim): The concatenation of the **Form ID** and the **Server
  Key ID**.
- `iat` (registered claim): The current timestamp.
- `exp` (registered claim): A short expiration timestamp (e.g. 1 minute from
  `iat`).
- `jti` (registered claim): A random nonce generated by the server to ensure
  the **API Challenge** can only be used once. This is stored in a key-value
  store with a short TTL.
- `nonce` (custom claim): A random nonce generated by the server.

The server returns the **API Challenge** to the client, which decodes (but does
not verify) it to get the `nonce`.

The client signs the nonce with the **Private Signing Key** and generates an
**API Challenge Response** from that signature, the **API Challenge**, the
**Form ID**, and the **Client Key ID**. The **API Challenge Response** is a
JSON object with this format:

```json
{
  "signature": "<nonce_signature>",
  "challenge": "<api_challenge>"
}
```

The **API Challenge Response** is then sent back to the server via an
unauthenticated API endpoint to exchange it for an **API Access Token**.

The server verifies:

- The signature of the **API Challenge** using the **Secret Server Key**
  associated with the `kid` to ensure the nonce the client signed is the same
  one it issued.
- The `exp` claim to ensure the **API Challenge** has not expired.
- The `jti` claim to ensure the **API Challenge** can only be used once.
- The signature of the signed nonce using the **Public Signing Key** associated
  with the **Form ID** and **Client Key ID**.

The server then deletes the `jti` from the key-value store.

The server exchanges the **API Challenge Response* with an **API Access
Token**. The **API Access Token** is a JWT signed with the **Secret Server
Key** via `HS256`. The **API Access Token** has the following claims:

- `kid` (header claim): The concatenation of the **Form ID** and the **Server
  Key ID**.
- `iss` (registered claim): The server's origin.
- `aud` (registered claim): The server's origin.
- `sub` (registered claim): The concatenation of the **Form ID** and the
  **Client Key ID**.
- `iat` (registered claim): The current timestamp.
- `exp` (registered claim): The time the **API Access Token** expires, which is
  configurable.

On subsequent authenticated API requests, the client includes the **API Access
Token** as a bearer token in the `Authorization` header, which the server
validates to authorize the request. The server validates:

- The signature of the **API Access Token** using the **Secret Server Key**
  associated with the `kid`.
- The `iss` matches the server's origin.
- The `aud` matches the server's origin.
- The **Form ID** in the `sub` matches the resource being requested.
- The `exp` claim to ensure the **API Access Token** has not expired.

## Key rotation

The server rotates the **Secret Server Key** associated with each **Form**
periodically using a cron job.

In the database, the server maintains a list of **Secret Server Keys** for each
**Form**. In practice, there will be two: The **Current Secret Server Key** and
the **Previous Secret Server Key**. Each key has a **Server Key ID** associated
with it.

Each rotation period, the **Current Secret Server Key** is demoted to the
**Previous Secret Server Key**, the **Previous Secret Server Key** is deleted,
and a new **Current Secret Server Key** is generated. The server always uses
the **Current Secret Server Key** to sign the **API Challenge** and **API
Access Token**.

The **Server Key ID** is included in JWTs in the `kid` header claim so the
server knows which **Secret Server Key** to use to verify the JWT.

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

Send a **Wrapped Private Primary Key** and its associated encrypted comment to
the server, associated with a **Form**.

```
POST /keys/:form_id
```

List the **Wrapped Private Primary Keys** associated with a **Form**, along
with their respective encrypted comments.

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
POST /tokens/:form_id/:client_key_id
```

## Glossary

- **Form**: A web form for collecting **Submissions** from users.
- **Submission**: Information encrypted locally with the **Public Client Key**
  and sent to the server.
- **Organizer**(s): The user who creates a **Form** and has access to its
  **Secret Link**(s).
- **Sharing Link**: A URL that can be followed to fill out a **Form**.
- **Secret Link**: A URL that can be used to decrypt and view **Submissions**.
- **Private Client Key**: A private key generated by the **Organizer** that is
  used to decrypt **Submissions**.
- **Public Client Key**: A public key generated by the **Organizer** that is
  used to encrypt **Submissions**.
- **Private Server Key**: A private key generated by the server that is used to
  validate **API Proofs**.
- **Public Server Key**: A public key generated by the server that is used to
  generate **API Proofs**.
- **Private Wrapping Key**: A private key generated by the client that is used
  to encrypt the **Private Client Key**, generating a **Wrapped Private Client
  Key**. This key forms part of a **Secret Link**.
- **Wrapped Private Client Key**: The **Private Client Key** encrypted with a
  **Public Wrapping Key**.
- **API Proof**: A random byte string encrypted with the **Private Wrapping
  Key** with the **Public Server Key** as its recipient. The **API Proof** is
  used to authenticate API requests.
- **Form ID**: A unique non-secret identifier for a form.
- **Client Key ID**: A unique non-secret identifier for a **Wrapped Private
  Client Key**.
- **Server Key ID**: A unique non-secret identifier for a **Private Server
  Key**.
- **Current Server Key Pair**: The current **Private Server Key** and **Public
  Server Key** in the key rotation.
- **Previous Server Key Pair**: The previous **Private Server Key** and
  **Public Server Key** in the key rotation.
- **Box**: An asymmetric encryption mechanism provided by
  [libsodium](https://doc.libsodium.org/public-key_cryptography/authenticated_encryption).
- **Sealed Box**: An asymmetric encryption mechanism provided by
  [libsodium](https://doc.libsodium.org/public-key_cryptography/sealed_boxes).
