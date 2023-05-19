# Load from image
FROM postgres:latest

# Set environment variables.
ENV POSTGRES_PASSWORD "bebra2"
ENV POSTGRES_USER admin
ENV POSTGRES_DB manuspect

# Expose a port for Postres.
EXPOSE 5432

# Specify volume for data persistence
VOLUME /var/lib/postgresql/data

# Copy initialization scrip.
COPY database_init/init.sql /docker-entrypoint-initdb.d/

