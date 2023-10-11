-- Your SQL goes here
CREATE TABLE users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL CHECK(length(username) >= 2 AND length(username) <=32),
    salted_hash BLOB(48)
);
