[![codecov](https://codecov.io/gh/Almogo97/zero2prod/graph/badge.svg?token=BC6FUNEIYO)](https://codecov.io/gh/Almogo97/zero2prod)

# Zero to Production in Rust

Repo following the book [Zero to Production in Rust](https://www.zero2prod.com/index.html?country=Spain&discount_code=VAT20&country_code=ES).

There are some minor and major differences with the book. Minor differences include some code organization and small decisions. While the major differences are:

1. Use of [axum](https://github.com/tokio-rs/axum) instead of [actix-web](https://github.com/actix/actix-web) as web framework. Because it looks like the new shiny better framework.
2. Deploy to [shuttle.rs](https://www.shuttle.rs/) instead of [digitalocean](https://www.digitalocean.com/). Because it is free.


# Installs

Consider using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) to install the following packages with `cargo binstall <package-name>` instead of `cargo install <package-name>`

Hot reload

```bash
cargo install cargo-watch
```

Test coverage. `RUST_LOG=trace` env variable is necessary to count log lines as covered. 

```bash
cargo install tarpaulin
```

Check dependencies for security vulnerabilities

```bash
cargo install cargo-audit
```

Explain macros

```bash
cargo install cargo-expand
```

SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Transform logs from JSON to readable lines

```bash
cargo install bunyan
```

Deploy app to [shuttle.rs](https://www.shuttle.rs/)

```bash
cargo install cargo-shuttle
```

# User stories


>As a blog visitor,\
I want to subscribe to the newsletter,\
So that I can receive email updates when new content is published on the blog.


>As the blog author,\
I want to send an email to all my subscribers,\
So that I can notify them when new content is published.

>As a subscriber,\
I want to be able to unsubscribe from the newsletter,\
So that I can stop receiving email updates from the blog.


# FAQ

## The reason behind the integration tests directory structure
https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html

## Deploying to Shuttle.rs

Had to rename the project because the name 'zero2prod' was already taken. Which sucks. Thank god I was able to maintain the library name.

Must have `SQLX_OFFLINE=true` in *.cargo/config.toml*

Must run `cargo sqlx prepare -- --tests` for it to prepare both main app queries and test queries. Actually deploy without running the tests `cargo shuttle deploy --no-test` because it will fail when it doesn't find a db to run tets, also how would that even work, does it maintain the same db for production?

Now you have to `SQLX_OFFLINE=false` whenever we want to use a live connection with our database. Which may kinda be a bummer, but also ensures we don't forget to run the prepare command before deploying to production... So not so bad I guess, kinda good actually.

cargo shuttle deploy --no-test
