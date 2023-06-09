-- Add migration script here
CREATE TABLE user_fishtype(
    user_id uuid REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    fishtype_id uuid REFERENCES fish_type (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (user_id, fishtype_id)
);

CREATE TABLE user_recipe(
    user_id uuid REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    recipe_id uuid REFERENCES recipe (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (user_id, recipe_id)
);

