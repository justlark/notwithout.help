# Not Without Help

🚧 This repo is under construction 🚧

A web app for connecting organizations with people that want to get involved.

How it works:

- Organizers create forms for volunteers to fill out.
- Volunteers answer some questions about the kinds of roles they're interested
  in and leave their contact information.
- Neither organizers nor volunteers need an account, email address, or phone
  number to use the app. Volunteers just follow a link to fill out the form,
  and organizers have a separate link to view the responses.
- Only the organizers can see the volunteers' responses. Neither the website
  owner nor the hosting provider have access to them.

## Deployment

The client app is automatically deployed to Cloudflare Pages on pushes to
`main`. To deploy the worker, run:

```shell
cd ./worker/
npm install
npx wrangler deploy
```

You can also build the client app locally like this:

```shell
cd ./client/
npm install
npm run build
```

## Development

To run the app locally, you'll need to spin up a local instance of the backend
worker. It must be running on port `8787`.

```shell
cd ./worker/
npm install
npx wrangler dev
```

Then you can start the local dev server.

```shell
cd ./client/
npm install
npm run dev
```

For testing the backend API locally (e.g. in Postman), you'll need an ed25519
keypair. Here is an example keypair you can use for testing.

Public key (raw):

```
fEPlSpHKftM+PYs+9himpcnry9hXdZJp7dniLAuypbo=
```

Private key (raw):

```
q3Xym5KF1OwPyE1+9ncNkgb1PZ93PyEarAE4wMxKe+A=
```

Public key (PEM):

```
-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAfEPlSpHKftM+PYs+9himpcnry9hXdZJp7dniLAuypbo=
-----END PUBLIC KEY-----

```

Private key (PEM):

```
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIKt18puShdTsD8hNfvZ3DZIG9T2fdz8hGqwBOMDMSnvg
-----END PRIVATE KEY-----
```

## Copyright

Copyright © 2024 Lark

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.

