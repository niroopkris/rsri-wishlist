-- Your SQL goes here
CREATE TABLE friendship (
	id SERIAL PRIMARY KEY,
	user1 VARCHAR NOT NULL REFERENCES users(user_id),
	user2 VARCHAR NOT NULL REFERENCES users(user_id),
	friend_status VARCHAR NOT NULL
);
