# Build stage
FROM rust:1.83-slim as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build your program for release
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/jobs /usr/local/bin/

# Set the startup command
CMD ["jobs"]