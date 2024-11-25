# Security

This document is an informal overview of how this app mitigates security risks.

## Disclaimer

**This system has not been audited for security.** If your safety or freedom
are at risk, do not depend on this software to protect you.

## Overview

This app allows users, called *organizers* to create web forms and request
information from others, called *submissions*. Submissions are encrypted
client-side such that they cannot be read by the server.

When an organizer creates a form, they are given two links: a *sharing link*
that can be used to fill out the form, and a *secret link* that can be used to
view the form submissions.

The *organizer's private key* is generated on the organizer's device, and its
corresponding public key is sent to the server when a form is created. The
*organizer's public key* is send to clients and used to encrypt submissions via
a libsodium [sealed
box](https://doc.libsodium.org/public-key_cryptography/sealed_boxes).

## Anatomy of a link

Sharing links have this format:

```
https://notwithout.help/share/#/<form_id>
```

And secret links have this format:

```
https://notwithout.help/view/#/<form_id>/<key_index>/<symmetric_key>
```

- `form_id`: The unique identifier for the form.
- `key_index`: A unique identifier for the `symmetric_key`.
- `symmetric_key`: A symmetric encryption key used to decrypt the organizer's
  private key.

The `symmetric_key` is stored in the URL fragment rather than the path or query
parameters so it's not leaked to the CDN.

The `form_id` is also stored in the URL fragment so the CDN does not know which
form a user is filling out.

## Access management

After creating a form, the organizer is given the sharing link and a secret
link. Organizers can create additional secret links as well. Secret links can
have comments attached to them and can be revoked at any time.

When a secret link is generated:

1. The client generates a random symmetric encryption key called the *symmetric
   wrapping key*.
2. The client uses the symmetric wrapping key to encrypt the organizer's
   private key via a libsodium [secret
   box](https://doc.libsodium.org/secret-key_cryptography/secretbox) to
   generate a *wrapped private key*.
3. A user-provided comment for the key is encrypted with the organizer's public
   key via a libsodium sealed box
4. The client sends the wrapped private key, along with the encrypted comment,
   to the server.
5. The server returns a unique ID for the wrapped private key called the *key
   index*.

When a secret link is used:

1. The client uses the form ID and the key index to request the corresponding
   wrapped private key from the server.
2. The client uses the symmetric wrapping key to decrypt the wrapped private
   key and reveal the organizer's private key.
3. The organizer's private key is used to authenticate with the API (see
   [Authentication](#authentication)) and decrypt the form submissions.

## Authentication

Some API endpoints require authentication. See the [API](#api) section for
details.

When a form is created:

1. The server generates a random 32-byte secret called the *API secret*.
2. The API secret is encrypted with the organizer's public key via a libsodium
   sealed box to form the *API challenge*. It is then stored in the database.
3. The API secret is hashed using the Argon2id password hashing algorithm and
   stored in the database. Because the API secret is randomly generated, a
   static salt is used rather than a separate salt per secret.
4. At this point, the API secret is dropped and zeroized.

When a client makes an authenticated API request:

1. The client uses the form ID to request the API challenge from the server.
2. The client uses the organizer's private key to decrypt the API challenge and
   reveal the API secret.
3. The API secret is included in requests as a bearer token.
4. The API secret is hashed and compared to the hash stored in the database
   using a constant-time algorithm.
5. If the hashed secrets match, the user is authenticated.

## API

This section describes the API endpoints exposed by the server, including which
are authenticated and unauthenticated.

### Authenticated endpoints

#### Request the encrypted submissions for a given form

```
GET /submissions/:form_id
```

#### Delete the form and all submissions

```
DELETE /forms/:form_id
```

#### Send a wrapped private key to the server

```
POST /keys/:form_id
```

#### List a form's wrapped private keys and their encrypted comments

```
GET /keys/:form_id
```

#### Revoke a wrapped private key

```
DELETE /keys/:form_id/:key_index
```

### Unauthenticated endpoints

#### Create a new form

```
POST /forms
```

#### Get a form by its ID

```
GET /forms/:form_id
```

#### Send an encrypted form submission

```
POST /submissions/:form_id
```

#### Get a wrapped private key by its key index

```
GET /keys/:form_id/:key_index
```
