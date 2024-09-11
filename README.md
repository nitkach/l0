## Setup
- add environment variable with url of your Postgres database `DATABASE_URL="postgres://{ username }:{ password }@localhost:5432/l0_data"` via .env file or pass it before running app
- run migrations via `sqlx migrate run` (run `cargo install sqlx-cli` if `sqlx` is not installed)
