-- Add migration script here
ALTER TABLE fish_type
ALTER COLUMN about
SET NOT NULL;
