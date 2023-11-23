set dotenv-load

alias t := test
alias d := dev
alias u := update
alias cm := create-migration

default:
    just --list

# Create a new SQLX migration file.
create-migration migration-name:
    #!/bin/bash
    sqlx migrate add {{migration-name}} 

# Migrate the local database with new migrations.
migrate-local:
    #!/bin/bash
    echo "migrating db."
    DATABASE_URL=$DATABASE_URL sqlx migrate run 

# Migrate the prod database with new migrations.
migrate-prod:
    #!/bin/bash
    echo "migrating prod db."
    DATABASE_URL=$PROD_DATABASE_URL sqlx migrate run

# Run SQLX to create query data for ci.
prepare:
    #!/bin/bash
    echo "Preparing query data."
    cargo sqlx prepare -- --lib

# Run all of the tests.
test:
    #!/bin/bash

    # ensures that Mac isn't limiting the amount of files allowed opened.
    ulimit -n 5000
    cargo t

# Update dependencies.
update:
    #!/bin/bash
    cargo update
    just test
    
# Start the Actix server and watch for changes.
dev:
    #!/bin/bash
    cargo watch -w src -x run &
    DEV_SERVER_PID=$! 

    trap "kill $DEV_SERVER_PID" SIGINT

    wait $DEV_SERVER_PID

# Populate the database with fish data.
populate:
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


# Initialize an instance of the database.
init-db:
    #!/usr/bin/env bash
    set -x
    set -eo pipefail

    if ! [ -x "$(command -v psql)" ]; then
      echo >&2 "Error: psql is not installed."
      exit 1
    fi

    if ! [ -x "$(command -v sqlx)" ]; then
      echo >&2 "Error: sqlx is not installed."
      echo >&2 "Use:"
      echo >&2 "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
      echo >&2 "to install it."
      exit 1
    fi

    DB_USER=${POSTGRES_USER:=postgres}
    DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
    DB_NAME="${POSTGRES_DB:=fish}"
    DB_PORT="${POSTGRES_PORT:=5432}"

    if [[ -z "${SKIP_DOCKER}" ]]
    then
        docker run \
          -e POSTGRES_USER=${DB_USER} \
          -e POSTGRES_PASSWORD=${DB_PASSWORD} \
          -e POSTGRES_DB=${DB_NAME} \
          -p "${DB_PORT}":5432 \
          -d postgres \
          postgres -N 1000
    fi

    # Keep pinging Postgres until it's ready to accept commands
    export PGPASSWORD="${DB_PASSWORD}"
    until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
      >&2 echo "Postgres is still unavailable - sleeping"
      sleep 1
    done

    >&2 echo "Postgres is up and running on port ${DB_PORT}!"

    export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
    sqlx database create
    sqlx migrate run

    just populate

# Update the digital ocean configurations after making a change to the spec file
# digitial-ocean-update:
# #!/bin/bash
# doctl apps update $DIGITAL_OCEAN_ID --spec=spec.yml

# Build the fishy-edge Docker image locally. 
docker-build:
    #!/bin/bash
    docker build --tag fishy-edge --file Dockerfile .

# Run the Docker image of fishy-edge at port 8000.
docker-run:
    #!/bin/bash
    docker run -p 8000:8000 fisy-edge
