pub static DOWN: &'static str = "-- This file should undo anything in `up.sql`
DROP TABLE users;";

pub static UP: &'static str = "-- Your SQL goes here
CREATE TABLE users (
  id int NOT NULL,
  email VARCHAR(128) UNIQUE NOT NULL,
  encrypted_password BLOB NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
  access_token varchar(32),
  last_access TIMESTAMP NOT NULL,
  PRIMARY KEY (id)
);

CREATE UNIQUE INDEX email_idx ON users(email);
CREATE UNIQUE INDEX access_token_idx ON users(access_token);
";
