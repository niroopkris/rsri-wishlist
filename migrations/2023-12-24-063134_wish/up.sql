-- Your SQL goes here
CREATE TABLE wish (
  id SERIAL PRIMARY KEY,
  wish_owner SERIAL NOT NULL REFERENCES users(user_id),
  title VARCHAR NOT NULL,
  descr TEXT NOT NULL,
  access_level INTEGER NOT NULL
);
