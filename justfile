set dotenv-load

alias t := test
alias d := dev
alias u := update
alias cm := create-migration

# List out available commands.
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
    echo "Testing..."
    cargo t

# Update dependencies and run the tests.
update:
    #!/bin/bash
    cargo update
    echo $'Dependencies updated!\n'
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
    scripts/populate_db.sh

# Initialize an instance of the database.
init-db:
    #!/usr/bin/env bash
    scripts/init_db.sh

# Create a new database instance with fish data.
create-db:
    #!/usr/bin/env bash
    just init-db
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
