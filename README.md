## Setup
- create `.env` file with `DATABASE_URL="postgres://{ username }:{ password }@localhost:5432/l0_data"` url to your Postgres database
- run migrations via `sqlx migrate run` (run `cargo install sqlx-cli` if `sqlx` is not installed)
