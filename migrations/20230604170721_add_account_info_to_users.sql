-- Add migration script here
ALTER TABLE users
ADD COLUMN first_name VARCHAR,
ADD COLUMN last_name VARCHAR;
