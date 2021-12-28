-- Your SQL goes here
ALTER TABLE Vahdit
ADD site_id Integer NOT NULL;

ALTER TABLE Blacklists
ADD site_id Integer NOT NULL;
