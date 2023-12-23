-- Your SQL goes here
CREATE TABLE wishlists (
  title VARCHAR NOT NULL PRIMARY KEY,
  list_desc TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE,
  user_id VARCHAR NOT NULL REFERENCES users(user_id)
);

