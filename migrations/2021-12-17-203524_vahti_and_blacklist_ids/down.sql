-- This file should undo anything in `up.sql`
CREATE TABLE TempVahdit(
    url TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    last_updated BIGINT NOT NULL
);
INSERT INTO TempVahdit (url, user_id, last_updated)
SELECT url, user_id, last_updated FROM Vahdit;
DROP TABLE Vahdit;
ALTER TABLE TempVahdit RENAME TO Vahdit;

CREATE TABLE TempBlacklist(
    user_id INTEGER NOT NULL,
    seller_id INTEGER NOT NULL
);
INSERT INTO TempBlacklist (user_id, seller_id)
SELECT user_id, seller_id FROM Blacklists;
DROP TABLE Blacklists;
ALTER TABLE TempBlacklist RENAME TO Blacklist;
