# Datastore

## .env

To have sqlx compile time query checks, the `.env` must have the database url provide the absolute path. Relative paths sometimes work, but are tricky to get working between normal compilation and test compilation across different test runner environments, and harder to re-use between the application and the sqlx cli, so creating your .env (in this directory) with an absolute path will cause the least headache. For example, it may look like this

```bash
DATABASE_URL="sqlite:/home/mike/git/delay-us-repay-us/crates/datastore/duru.db"
```

## Sqlx cli

For manual actions like adding and running migrations, it's easier to cd into this directory, and run the `sqlx` cli commands here as they will load the `DATABASE_URL` from the `.env`.

For example, to run migrations, cd here and try

```bash
sqlx migrate run
```

Similarly, if needing to delete the database file and recreate from scratch for any reason, cd here and try

```bash
sqlx db drop
sqlx db create
sqlx migrate run
```

### Migrations

As mentioned above, migrations can be run manually. The datastore also runs migrations upon instantiation, to ensure none are forgotten (and to run them on the ephemeral test database). This could happen if a migration file is added that is not then relied upon by any query (and thus would not be a compile error until it is applied).

### Prepare (allowing compile time sql query checks in CI)

In CI, there is no database to check against at compile time, so we must run `cargo sqlx prepare` (must run as a `cargo` subcommand) from within this directory (with the database url available) to generate files in `.sqlx/` which CI can then use to pass compilation. Prek will check if this is up to date.
