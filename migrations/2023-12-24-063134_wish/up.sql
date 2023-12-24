-- Your SQL goes here
CREATE TABLE wish (
  id SERIAL PRIMARY KEY,
  wish_owner VARCHAR NOT NULL REFERENCES users(user_id),
  title VARCHAR NOT NULL,
  descr TEXT NOT NULL,
  access_level VARCHAR NOT NULL
);
