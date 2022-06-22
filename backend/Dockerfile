FROM rust:bullseye as builder
RUN rustup default nightly

WORKDIR /app
# copy the workspace toml
COPY Cargo.toml ./

# copy the workspace members to the workspace
COPY secrets ./secrets

WORKDIR /app/api
COPY ./api/Cargo.toml ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

# Copy the real source code over
COPY api /app/api
RUN touch src/main.rs

RUN cargo build --release

# == == ==
# Copy the executable and extra files ("static") to an empty Docker image
FROM debian:bullseye
COPY --from=builder /app/target/release/ ./app
CMD [ "./app/api" ]