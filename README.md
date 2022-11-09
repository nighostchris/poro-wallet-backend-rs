# PoroWallet Backend in Rust

Actix + SQLx backend server for PoroWallet frontend query. Rust version for original TS version poro-wallet-backend.

## Local Environment Setup

### Running service

```bash
cargo run
```

### Database Migrations

```bash
# Command line tool for managing migrations
cargo install sqlx-cli
# Creating reversible migrations in migrations directory
sqlx migrate add -r <name>
# Apply migration (default to be using .env in same directory you running the command)
sqlx migrate run
# Apply migration with explicit database url
sqlx migrate run --database-url postgresql://postgres:postgres@localhost:5432/postgres
# Revert migration
sqlx migrate revert
```
