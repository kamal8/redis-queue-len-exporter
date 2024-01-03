# Tells docker to use the latest Rust official image
FROM rust:latest
# Copy our current working directory into the container
COPY ./ ./
# Create the release build
RUN cargo build --release

# Expose the port our app is running on
EXPOSE 8000
# Run the application!
CMD ["./target/release/redis-queue-exporter"]