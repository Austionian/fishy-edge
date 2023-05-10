-- Add migration script here
ALTER TABLE users
ADD COLUMN image_url TEXT;
