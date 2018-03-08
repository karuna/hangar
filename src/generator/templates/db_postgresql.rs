pub static DOWN_00: &'static str = "DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
";

pub static UP_00: &'static str = "-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
";

pub static DOWN_01: &'static str = "-- This file should undo anything in `up.sql`
DROP TABLE users;
";

pub static UP_01: &'static str = "-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(128) UNIQUE NOT NULL,
  encrypted_password BYTEA NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  access_token varchar(32) NULL,
  last_access TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

CREATE UNIQUE INDEX email_idx ON users(email);
CREATE UNIQUE INDEX access_token_idx ON users(access_token);
";
