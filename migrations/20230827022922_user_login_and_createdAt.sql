-- Add migration script here
ALTER TABLE users
ADD COLUMN created_at TIMESTAMPTZ,
ADD COLUMN latest_login TIMESTAMPTZ;
