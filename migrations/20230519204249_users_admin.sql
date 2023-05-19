-- Add migration script here
ALTER TABLE users
ADD COLUMN is_admin BOOLEAN;

ALTER TABLE users
ADD CONSTRAINT unique_email UNIQUE (email);
