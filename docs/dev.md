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

## Running the app

```shell
$ cargo run api
```

## (re-)building the Web UI

```shell
$ cd web/
$ npx vite build
# or
$ npx vite build --watch
```

The API will serve the web UI, no need to run a server yourself.

A development build of the API code will not slipstream the content of `./web/build` into the binary,
and instead use the folder as-is, so a `build --watch` works fine.
