#!/usr/bin/env bash
# After creating the db, run this from the project's root to populate it with data.
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy users from "migration_data/users.csv" DELIMITER ',' CSV HEADER;"
# psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy fish_type from "migration_data/fish_type.csv" DELIMITER ',' CSV HEADER;"
# psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy fish from "migration_data/fish.csv" DELIMITER ',' CSV HEADER;"
# psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy recipe from "migration_data/recipe.csv" DELIMITER ',' CSV HEADER;"
# psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy fishtype_recipe from "migration_data/fishtype_recipe.csv" DELIMITER ',' CSV HEADER;"
# MIGHT NOT NEED
# psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy fish_type_recipe from "migration_data/fish_type_recipe.csv" DELIMITER ',' CSV HEADER;"
psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy user_fishtype from "migration_data/user_fishtype.csv" DELIMITER ',' CSV HEADER;"
psql "postgres://postgres:password@docker.local:5432/fish" -c "\copy user_recipe from "migration_data/user_recipe.csv" DELIMITER ',' CSV HEADER;"
