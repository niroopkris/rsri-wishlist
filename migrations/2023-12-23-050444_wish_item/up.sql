-- Your SQL goes here
CREATE TABLE wish_item (
  id SERIAL PRIMARY KEY,
  item_name VARCHAR NOT NULL,
  notes TEXT NOT NULL,
  wishlist VARCHAR NOT NULL REFERENCES wishlists(title)
);