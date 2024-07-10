-- Your SQL goes here
CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  passwd VARCHAR NOT NULL
);
