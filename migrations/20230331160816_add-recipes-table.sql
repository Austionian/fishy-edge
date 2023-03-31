-- Add migration script here

CREATE TABLE recipe(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    ingredients TEXT ARRAY,
    steps TEXT ARRAY
);
