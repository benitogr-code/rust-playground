CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL
);

CREATE UNIQUE INDEX username_unique_idx ON users(username);
