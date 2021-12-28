-- This file should undo anything in `up.sql`
ALTER TABLE Vahdit
DROP COLUMN site_id;

ALTER TABLE Blacklists
DROP COLUMN site_id;
