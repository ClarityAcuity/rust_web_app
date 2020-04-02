# SetUp

## [Instasll Rust](https://www.rust-lang.org/tools/install)

```shell
rustup toolchain install nightly
rustup default nightly
```

## [Install SQLite3](https://sqlite.org/index.html)

```shell
sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev
```

## Install wasm toolchain

```shell
rustup target add wasm32-unknown-unknown
```

## Build

```shell
cargo install cargo-make
cargo make
```

## Schema migration

```shell
cargo install diesel_cli --no-default-features --features sqlite
cd backend
diesel migration redo
```

## Add data

```shell
cargo run -p backend --bin todo new 'input any string'
```

## Start Backend

API serve at http://localhost:8000/tasks/

```shell
cargo run -p backend --bin backend
```

## Serve Frontend

Web page serve at http://localhost:9090/

```shell
cd frontend
cargo install microserver
microserver
```
