# CRUD app scaffolding with Axum + SQLX + SQLite

A demo project to showcase how I would structure the scaffolding for a CRUD app with the following features:

- Layered architecture
- App configuration
- Tracing logs
- Mocks
- Test setup

## Disclaimer

This repository was created for demo purposes, and it's not meant to reflect a production-level project.

## Getting started

1. Install [`sqlx-cli`](https://lib.rs/crates/sqlx-cli).
2. Create the database.

```console
sqlx db create
```

3. Run the database migrations.

```console
sqlx migrate run
```

## Usage

Start the server:

```console
cargo run
```

Run the tests:

```console
cargo test
```
