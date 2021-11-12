-- Add migration script here
CREATE TABLE Vahdit(
    url TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    last_updated BIGINT NOT NULL
);

