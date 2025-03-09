# Postgres setup

```sh
# The postgres user is the default superuser for PostgreSQL databases and is usually created during PostgreSQL installation.
# psql is the command-line client for PostgreSQL
sudo -u postgres psql


CREATE ROLE my-app-user WITH LOGIN PASSWORD 'my-pwd'

CREATE DATABASE my-app-db WITH OWNER='my-app-user'

\q


# Login to db
psql -U my-app-db

CREATE TABLE tasks(
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
);

```
