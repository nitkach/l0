## Setup
- (optional) if you want a different `PG_USER`, `PG_PASSWORD`, `PG_DB`, then change the environment variables in the `.env` file. Also update `DATABASE_URL`: `postgres://{ your PG_USER }:{ your PG_PASSWORD }@localhost:5432/{ your PG_DB }`
- run migrations via `sqlx migrate run` (run `cargo install sqlx-cli` if `sqlx` is not installed)

## Run
- run `docker compose up` command to run app

## Testing
- if the app is running, you can test its API via scripts in `./scripts` folder
    - create orders: `bash ./scripts/test_post_api.sh`
    - get orders: `bash ./scripts/test_get_api.sh`
    - list orders: `bash ./scripts/test_list_api.sh`
    - delete orders: `bash ./scripts/test_delete_api.sh`
