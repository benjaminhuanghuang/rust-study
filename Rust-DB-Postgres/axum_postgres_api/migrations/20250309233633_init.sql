-- Add migration script here
CREATE TABLE tasks(
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
)