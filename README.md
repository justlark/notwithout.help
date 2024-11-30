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

In this [`./tests/`](./tests/) directory you'll find a CLI tool that provides
helpers for performing the cryptographic operations necessary to authenticate
with the backend worker. This is useful for debugging the worker locally using
a tool like Postman or Hoppscotch.

To use the tool, [install Rust](https://www.rust-lang.org/tools/install) and
run:

```shell
cd ./tests/
cargo run -- --help
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

