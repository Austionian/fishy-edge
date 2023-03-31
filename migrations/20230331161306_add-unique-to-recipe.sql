-- Add migration script here
ALTER TABLE recipe
ADD CONSTRAINT unique_name UNIQUE (name);
