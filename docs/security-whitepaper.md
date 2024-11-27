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

1. The client retrieves and decrypts the **Private Client Key** as described in
   the [Retrieving the private client key](#retrieving-the-private-client-key)
   section.
2. The client generates a new random (**Private Wrapping Key**, **Public
   Wrapping Key**) pair.
3. The client uses the new **Public Wrapping Key** to encrypt the **Private
   Client Key** via a **Sealed Box** to generate a new **Wrapped Private Client
   Key**.
4. A user-provided comment for the key is encrypted with the **Public Client
   Key** via a **Sealed Box** the only the **Private Client Key** can decrypt.
5. The client sends the new **Wrapped Private Client Key**, the new **Public
   Wrapping Key**, and the encrypted comment to the server via an authenticated
   endpoint.
6. The server generates a new **Client Key ID** for the **Wrapped Private
   Client Key**. It stores the **Wrapped Private Client Key**, **Public
   Wrapping Key**, encrypted comment, and **Client Key ID** in the database.
7. The server returns the new **Client Key ID** to the client.
8. The **Form ID**, new **Client Key ID**, and new **Private Wrapping Key**
   form the new **Secret Link**.

A **Secret Link** can be revoked by the **Organizer** via an authenticated
endpoint. This deletes the **Wrapped Private Client Key** from the database.

Note that once a **Secret Link** has been used to reveal the **Private Client
Key**, while revoking it will deny API access, it will not deny the ability to
decrypt **Submissions** if the ciphertext is leaked.

## Retrieving the private client key

A **Secret Link** can be used to retrieve and decrypt the **Private Client
Key**.

1. The client uses the **Form ID**, **Client Key ID**, and **Private Wrapping
   Key** encoded in the **Secret Link** to call an authenticated API endpoint
   (as described in the [Authentication](#authentication) section) to get the
   **Wrapped Private Client Key**.
2. The client uses the **Private Wrapping Key** to decrypt the **Wrapped
   Private Client Key** and reveal the **Private Client Key**.

From here, the client can call a different authenticated API endpoint to get
the list of encrypted **Submissions**, which can be decrypted using the
**Private Client Key**.

## Authentication

Some API endpoints require authentication. See the [API](#api) section for
details.

To make an authenticated API request:

1. The client uses the **Form ID** to request the **Public Server Key** and
   **Server Key ID** from the server.
2. The client uses the **Private Wrapping Key** and the **Public Server Key**
   to encrypt a random byte string via a libsodium
   [**Box**](https://doc.libsodium.org/public-key_cryptography/authenticated_encryption)
   to generate the **API Proof**.
3. The **API Proof** is concatenated with the **Server Key ID** and **Client
   Key ID** and included in the API request as a bearer token.
4. The server uses the **Private Server Key** associated with the **Server Key
   ID** and the **Public Wrapping Key** associated with the **Client Key ID**
   to validate the **API Proof**.
5. If the **API Proof** is validated, the server authorizes the request.

## Key rotation

The server rotates the **Private Server Key** associated with each **Form**
periodically using a cron job.

In the database, the server maintains a list of (**Private Server Key**,
**Public Server Key**) pairs for each **Form**. In practice, there will be two:
The **Current Server Key Pair** and the **Previous Server Key Pair**. Each pair
has a **Server Key ID** associated with it.

Each rotation period, the **Current Server Key Pair** is demoted to the
**Previous Server Key Pair**, the **Previous Server Key Pair** is deleted, and
a new **Current Server Key Pair** is generated. When the server returns the
**Public Server Key** to the client over the API, it always returns the one
from the **Current Server Key Pair**.

When the server returns the **Public Server Key** to the client over the API,
it also returns its associated **Server Key ID**. The **Server Key ID** is
included in the bearer token used to authenticate API requests so that the
server knows which **Private Server Key** to validate the **API Proof**
against.

## API

This section lists the authenticated and unauthenticated API endpoints exposed
by the server.

### Authenticated endpoints

Request the ciphertext of the encrypted **Submissions** for a **Form**.

```
GET /submissions/:form_id
```

Delete the **Form** from the database, along with all its associated
**Submissions**, **Wrapped Private Client Keys**, and **Private Server Keys**.

```
DELETE /forms/:form_id
```

Get a **Wrapped Private Client Key** by its **Client Key ID**.

```
GET /keys/:form_id/:key_id
```

Send a **Wrapped Private Client Key** and its associated encrypted comment to
the server, associated with a **Form**.

```
POST /keys/:form_id
```

List the **Wrapped Private Client Keys** associated with a **Form**, along with
their respective encrypted comments.

```
GET /keys/:form_id
```

Revoke a secret link by deleting its associated **Wrapped Private Client Key**.

```
DELETE /keys/:form_id/:key_id
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
