# App components

There are multiple technologies at play that compose the Logistics Inventory app.

1. PostgreSQL database  
   This holds all the stateful data of the application
1. Logistics Inventory API  
   This is a backend application written in Rust.  
   A good intro to Rust is the [Rust Handbook](https://doc.rust-lang.org/book/).  
   It uses multiple notable libraries, each with their own purpose
   1. [Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) as an async task scheduler
   1. [Axum](https://docs.rs/axum/latest/axum/) to serve an API
   1. [Diesel](https://diesel.rs/guides/all-about-selects) to interact with the database.
   1. [Tracing](https://github.com/tokio-rs/tracing) for logging.
1. Web UI  
   This is a [SvelteKit](https://svelte.dev/tutorial/svelte/welcome-to-svelte) project that builds to a static web application.  
   It is slipstreamed into the API binary during compilation (except during development).
