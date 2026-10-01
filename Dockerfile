FROM rust:1.90-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY seed.json ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/backend /usr/local/bin/backend
ENV DATABASE_PATH=/app/articles.db
EXPOSE 3000
CMD ["backend"]
