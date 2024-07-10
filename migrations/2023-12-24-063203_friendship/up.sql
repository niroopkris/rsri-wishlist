-- Your SQL goes here
CREATE TABLE friendship (
	id SERIAL PRIMARY KEY,
	user1 SERIAL NOT NULL REFERENCES users(user_id),
	user2 SERIAL NOT NULL REFERENCES users(user_id),
    friend_status INTEGER NOT NULL
);
