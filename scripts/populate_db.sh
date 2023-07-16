#!/usr/bin/env bash
# After creating the db, run this from the project's root to populate it with data.
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

psql "postgres://postgres:password@localhost:5432/fish" -c "\copy fish_type from "scripts/basic_data/fish_types.csv" DELIMITER ',' CSV HEADER;"
psql "postgres://postgres:password@localhost:5432/fish" -c "\copy fish from "scripts/basic_data/fishs.csv" DELIMITER ',' CSV HEADER;"
psql "postgres://postgres:password@localhost:5432/fish" -c "\copy recipe from "scripts/basic_data/recipes.csv" DELIMITER ',' CSV HEADER;"
psql "postgres://postgres:password@localhost:5432/fish" -c "\copy fishtype_recipe from "scripts/basic_data/fish_recipe.csv" DELIMITER ',' CSV HEADER;"
