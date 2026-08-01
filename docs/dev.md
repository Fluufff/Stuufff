# Development guide

This repository uses Rust, and leverages the
default cargo toolset (e.g. `cargo build`).

This application however needs supporting services, like a database, and Google access tokens.

To facilitate these extra needs, this project uses [just](https://just.systems/man/en/).

## Running a db instance

```shell
$ just run-db
```

This will generate local credentials if needed, and (re)-start the database.

## Setting up the db

```shell
$ just db-migrate
```

## (re-)building the Web UI

```shell
$ cd web/
$ npm i
$ npx vite build
# or
$ npx vite build --watch
```

The API will serve the web UI, no need to run a server yourself.

A development build of the API code will not slipstream the content of `./web/build` into the binary,
and instead use the folder as-is, so a `build --watch` works fine.

## Setting up libpq (PostgreSQL) and diesel

```shell
$ brew install libpq
$ cargo install diesel_cli --no-default-features --features postgres
```

## Running the app

```shell
$ mkdir media
$ cargo run api --no-auth
```

### Running the app with Google authentication on

You will need to set up
- `local_secrets/oauth_client`
- `local_secrets/oauth_secret`
- `local_secrets/groups-reader.json`

These file you will need to get from your HoD.

You can then run
```shell
$ cargo run api
```
