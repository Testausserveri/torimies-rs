-- Add migration script here
CREATE TABLE Blacklist(
    user_id INTEGER NOT NULL,
    seller_id INTEGER NOT NULL
);
