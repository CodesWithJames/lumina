FROM rust:bullseye as builder
RUN rustup default nightly

WORKDIR /app
# # Build the executable
# FROM rust:bullseye as builder

# WORKDIR /app

# COPY Cargo.toml ./

# RUN mkdir src/
# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN cargo build --release

# COPY ./src ./src

# RUN touch src/main.rs
# RUN cargo build --release

# # == == ==
# # Copy the executable and extra files ("static") to an empty Docker image
# FROM debian:bullseye
# COPY --from=builder /app/target/release/ ./app
# CMD [ "./app/your_app_name" ]