-- Your SQL goes here
CREATE TABLE TempVahdit(
    id INTEGER PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    last_updated BIGINT NOT NULL
);

INSERT INTO TempVahdit (url, user_id, last_updated)
SELECT url, user_id, last_updated FROM Vahdit;
DROP TABLE Vahdit;
ALTER TABLE TempVahdit RENAME TO Vahdit;

CREATE TABLE TempBlacklist(
    id INTEGER PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL,
    seller_id INTEGER NOT NULL
);
INSERT INTO TempBlacklist (user_id, seller_id)
SELECT user_id, seller_id FROM Blacklist;
DROP TABLE Blacklist;
ALTER TABLE TempBlacklist RENAME TO Blacklists;
