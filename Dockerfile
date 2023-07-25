ARG RUST_VERSION=1.70.0
ARG DEBIAN_VERSION=11.6

# all credit goes to https://fasterthanli.me/articles/remote-development-with-rust-on-fly-io#what-the-heck-is-fly-io-for-even
# for an an awesome walkthrough of docker files for rust, this is more or less a direct copy pasta with a few minor tweaks

# after containerization, this manages to come in at a whopping ~163mb, still a bit to we could optimize but this should do for now

# stage one - copy over our build files for compilation, including workspace and .env files
FROM rust:${RUST_VERSION}-slim-bullseye AS build

WORKDIR /app

COPY . .

# on rebuilds, we explicitly cache our rust build dependencies to speed things up
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes pkg-config libssl-dev nodejs; \
    apt clean autoclean; \
    apt autoremove --yes;

# replace the relevant keys in our manifest for deployment to fly
RUN sed -i 's/env = "DEV"/env = "PROD"/' ./Cargo.toml

# install all the required leptos tools
RUN rustup target add wasm32-unknown-unknown; \
    cargo install cargo-leptos; \
    npm install -g sass; \
    cargo leptos build --release; \
    objcopy --compress-debug-sections target/server/release/realworld-leptos ./realworld-leptos;

# stage two - we'll utilize a second container to run our built binary from our first container - slim containers!
FROM debian:${DEBIAN_VERSION}-slim as deploy

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends openssl ca-certificates; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/

WORKDIR /deploy

# Must match your `output-name` from the `metadata.leptos` until the next release
ENV LEPTOS_OUTPUT_NAME="realworld-leptos"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"

# copy over build artifacts from the build stage
COPY --from=build /app/realworld-leptos ./
COPY --from=build /app/target/site ./site

EXPOSE 80
EXPOSE 443

CMD ["./realworld-leptos"]
