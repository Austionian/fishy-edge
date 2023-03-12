-- Add migration script here
CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL, 
    name TEXT
);
