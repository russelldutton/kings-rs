# Kings Backend

A Rust + Axum stack, with sqlx for DB Queries.

## DB Migrations

If your database has not been created already, run the following command to create the database:

```
cargo sqlx database create
```

Similarly, `cargo sqlx database drop` will delete the whole database. Of course, you could also just delete the file yourself.

To run migrations

```
cargo sqlx migrate run
```

This will run all migrations
