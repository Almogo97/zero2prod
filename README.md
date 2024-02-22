# Zero to Production in Rust

Repo following the [book](https://www.zero2prod.com/index.html?country=Spain&discount_code=VAT20&country_code=ES) using [axum](https://github.com/tokio-rs/axum) insted of [actix-web](https://github.com/actix/actix-web) as web framework.

# Installs

Hot reload

```bash
cargo install cargo-watch
```

Test coverage

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
