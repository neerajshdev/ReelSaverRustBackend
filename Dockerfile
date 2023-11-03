# --- Build Stage ---
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin api
WORKDIR /app

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now copy your source code
COPY src src

# Build your application
RUN rm ./target/release/deps/api*
RUN cargo build --release

# --- Production Stage ---
FROM debian:buster-slim
ARG APP=/usr/src/app

# Create app directory
RUN mkdir -p ${APP}

WORKDIR ${APP}

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api ${APP}/api

# Set environment variables
ENV TZ=Etc/UTC \
    APP_USER=appuser

# Create a new user to run our application
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

# Run the binary
CMD ["./api"]
