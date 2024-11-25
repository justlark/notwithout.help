# Security

This document is an informal overview of how this app mitigates security risks.

## Disclaimer

**This system has not been audited for security.** If your safety or freedom
are at risk, do not depend on this software to protect you.

## Overview

This app allows users, called **Organizers** to create end-to-end encrypted web
forms, collecting **Submissions** from others. **Submissions** are encrypted
client-side such that they cannot be read by the server.

When an **Organizer** creates a **Form**, they are given two links: a **Sharing
Link** that can be followed to fill out the **Form**, and a **Secret Link**
that can be used to view the **Submissions**.

The **Organizer's Private Key** is generated on the organizer's device, and its
corresponding public key is sent to the server when a **Form** is created. The
**Organizer's Public Key** is then sent to clients and used to encrypt
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
- `key_id`: A **Key ID**, a unique identifier for a **Wrapped Private Key**.
- `key`: A **Symmetric Wrapping Key**, used to decrypt a **Wrapped Private
  Key** to reveal the **Organizer's Private Key** (see [Access
  management](#access-management)).

The `key` is stored in the URL fragment rather than the path or query
parameters so it's not leaked to the CDN.

The `form_id` is also stored in the URL fragment so the CDN does not know which
**Form** a user is filling out.

## Access management

After creating a **Form**, the **Organizer** is given the **Sharing Link** and
a **Secret Link**. Organizers can create additional secret links as well.
Secret links can have comments attached to them and can be revoked at any time.

When a **Secret Link** is generated:

1. The client generates a random symmetric encryption key called the
   **Symmetric Wrapping Key**.
2. The client uses the **Symmetric Wrapping Key** to encrypt the **Organizer's
   Private Key** via a libsodium [**Secret
   Box**](https://doc.libsodium.org/secret-key_cryptography/secretbox) to
   generate a **Wrapped Private Key**.
3. A user-provided comment for the key is encrypted with the **Organizer's
   Public Key** via a **Sealed Box**.
4. The client sends the **Wrapped Private Key**, along with the encrypted
   comment, to the server.
5. The server returns a unique ID for the **Wrapped Private Key** called the
   **Key ID**.

When a **Secret Link** is used:

1. The client uses the **Form ID** and the **Key ID** to request the
   corresponding **Wrapped Private Key** from the server.
2. The client uses the **Symmetric Wrapping Key** to decrypt the **Wrapped
   Private Key** and reveal the **Organizer's Private Key**.
3. The **Organizer's Private Key** is used to authenticate with the API (see
   [Authentication](#authentication)) and decrypt the **Submissions**.

Note that once an adversary has a valid **Secret Link** and has extracted the
**Organizer's Secret Key** from it, revoking that **Wrapped Secret Key** does
not deny them API access or the ability to decrypt **Submissions**. All
revoking a **Wrapped Secret Key** does is delete it from the database,
preventing future access to it.

## Authentication

Some API endpoints require authentication. See the [API](#api) section for
details.

When a **Form** is created:

1. The server generates a random 32-byte secret called the **API Secret**.
2. The **API Secret** is encrypted with the **Organizer's Public Key** via a
   **Sealed Box** to form the **API Challenge**. It is then stored in the
   database.
3. The **API Secret** is hashed using the Argon2id password hashing algorithm
   and stored in the database. Because the **API Secret** is randomly
   generated, a static salt is used rather than a separate salt per secret.
4. At this point, the **API Secret** is dropped and zeroized.

When a client makes an authenticated API request:

1. The client uses the **Form ID** to request the **API Challenge** from the
   server.
2. The client uses the **Organizer's Private Key** to decrypt the **API
   Challenge** and reveal the **API Secret**.
3. The **API Secret** is included in requests as a bearer token.
4. The **API Secret** is hashed and compared to the hash stored in the database
   using a constant-time algorithm.
5. If the hashed secrets match, the user is authenticated.

## API

This section lists the authenticated and unauthenticated API endpoints exposed
by the server.

### Authenticated endpoints

Request the encrypted **Submissions** for a **Form**.

```
GET /submissions/:form_id
```

Delete the **Form** from the database, along with all its associated
**Submissions**, and **Wrapped Private Keys**.

```
DELETE /forms/:form_id
```

Associate a **Wrapped Private Key** and its associated encrypted comment with a
**Form**.

```
POST /keys/:form_id
```

List the **Wrapped Private Keys** associated with a **Form**, along with their
respective encrypted comments.

```
GET /keys/:form_id
```

Revoke a **Wrapped Private Key**.

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

Get a **Wrapped Private Key** by its **Key ID**.

```
GET /keys/:form_id/:key_id
```
