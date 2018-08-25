A little microservice program I'm trying to write, for practice.

## Compile & Run

Pre-reqs:

* [Rust](https://rustup.rs)
* [Postgres](https://www.postgresql.org/)

Install:
```sh
cargo install diesel_cli --no-default-features --feature postgres
```

Migrate Up:
```sh
diesel migration run
```

Migrate Down:
```sh
diesel migration revert
```

Run:
```sh
cargo run
```

## Endpoints
| endpoint                  | http method | returns                             |
|---------------------------|-------------|-------------------------------------|
| /animals/\<name>/\<species> | POST        | newly created item                  |
| /animals                  | GET         | everything we have (up to 20 items) |

