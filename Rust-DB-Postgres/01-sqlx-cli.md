# Sqlx Database management

```sh
# Creates the database if it does not exist, using the config in .env
sqlx database create

# It is a shortcut that combines of `sqlx database create` and `sqlx migrate run`
sqlx database setup  # Runs migrations


# --------- Migration
# Creates a new migration.
sqlx migrate add <name>
# create the migrations/ folder in your project:
sqlx migrate add init


# Run all pending migrations from the migrations/ folder.
sqlx migrate run

# Check applied migrations:
sqlx migrate info


# undo the last migration:
sqlx migrate revert


#---------- Create new table
sqlx migrate add create_projects_table
sqlx migrate run
```

## Migration up, down

migrations/YYYYMMDDHHMMSS_init.up.sql:

```sql
CREATE TABLE users (
id INTEGER PRIMARY KEY AUTOINCREMENT,
name TEXT NOT NULL,
email TEXT UNIQUE NOT NULL
);
```

migrations/YYYYMMDDHHMMSS_init.down.sql:

```sql
DROP TABLE users;
```
