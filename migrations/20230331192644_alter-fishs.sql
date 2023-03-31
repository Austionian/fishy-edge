-- Add migration script here

CREATE TABLE fishtype_recipe(
    fishtype_id uuid REFERENCES fish_type (id) ON UPDATE CASCADE ON DELETE CASCADE,
    recipe_id uuid REFERENCES recipe (id) ON UPDATE CASCADE,
    PRIMARY KEY (fishtype_id, recipe_id)
);
