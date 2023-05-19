# Use the official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the rest of the project files to the container
COPY ./ ./

# Build the application
RUN cargo build --release

# Set the command to run your application
CMD ["cargo", "run", "--release"]
