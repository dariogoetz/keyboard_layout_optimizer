FROM rust:latest as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new layouts_webservice
WORKDIR /usr/src/app/webui/layouts_webservice

# COPY Cargo.toml ../../Cargo.toml
COPY keyboard_layout ../../keyboard_layout
COPY layout_evaluation ../../layout_evaluation
COPY webui/layouts_webservice/Cargo.toml Cargo.toml
COPY webui/layouts_webservice/Cargo.lock Cargo.lock

# # Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN cargo build --release

RUN rm -rf src

# Copy the rest
COPY webui/layouts_webservice/db db
COPY webui/layouts_webservice/src src

# Make sure the binary is recompiled even if the main is older
RUN touch src/main.rs

# Build (install) the actual binaries
RUN cargo build --release

# Runtime image
FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app/webui/layouts_webservice

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/webui/layouts_webservice/target/release/layouts_webservice /app/webui/layouts_webservice/layouts_webservice

COPY ngrams /app/ngrams
COPY config /app/config
COPY webui/layouts_webservice/Rocket.toml /app/webui/layouts_webservice/Rocket.toml
COPY webui/layouts_webservice/static /app/webui/layouts_webservice/static

EXPOSE 8080
CMD ["./layouts_webservice"]
