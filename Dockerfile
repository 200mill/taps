# Full multi-stage build — for local `docker build --build-arg BIN=<tap-name> .`
FROM rust:1-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --workspace --release

FROM debian:bookworm-slim AS runtime
ARG BIN

RUN if [ "$BIN" = "youtube-tap" ]; then \
      apt-get update && \
      apt-get install -y --no-install-recommends python3 yt-dlp && \
      rm -rf /var/lib/apt/lists/*; \
    fi

COPY --from=builder /app/target/release/${BIN} /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
