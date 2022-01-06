-- Your SQL goes here
ALTER TABLE Vahdit
ADD site_id Integer NOT NULL DEFAULT 1;

ALTER TABLE Blacklists
ADD site_id Integer NOT NULL DEFAULT 1;
