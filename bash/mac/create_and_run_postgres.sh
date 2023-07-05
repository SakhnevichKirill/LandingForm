#!/bin/zsh

# Load environment variables from a separate file.
source ./bash/mac/env.sh

# Stop the running container with postgres.
docker stop postgres-db-test-landing-form

# Remove the existing docker container with postgres.
docker rm postgres-db-test-landing-form

# Remove the existing image with postgres.
docker rmi postgres-db-test-landing-form

# Build a new image with postgres.
docker build -t postgres-db-test-landing-form -f Docker/postgres.dockerfile .

# Run the created image.
docker run -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -d -p 5432:5432 --name postgres-db-test-landing-form postgres-db-test-landing-form