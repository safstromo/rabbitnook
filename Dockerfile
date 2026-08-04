# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder
ENV LEPTOS_TAILWIND_VERSION=v4.3.3

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

# The builder is alpine, so the binary is musl-linked and needs nothing more
# than a bare alpine to run - no reason to ship the whole Rust toolchain.
FROM alpine:3 AS runner

WORKDIR /app

COPY --from=builder /work/target/release/rabbitnook /app/
COPY --from=builder /work/target/site /app/site
# Not cruft: get_configuration() reads [package.metadata.leptos] at runtime.
COPY --from=builder /work/Cargo.toml /app/

EXPOSE 3000
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/rabbitnook"]
