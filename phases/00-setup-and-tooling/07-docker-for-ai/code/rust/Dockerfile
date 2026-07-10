FROM rust:1.97.0-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p app

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/app /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
