# Build Stage
FROM rust:1.65.0 as builder

RUN USER=root cargo new --bin daxx
WORKDIR /daxx
COPY ./Cargo.toml ./Cargo.toml

# Build empty app with downloaded dependencies to produce a stable image layer for the next build
RUN cargo build --release

# Build the web app with your code
RUN rm src/*.rs
COPY ./src ./src
RUN cargo build --release


# Final Stage
FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8494

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /daxx/target/release/daxx ${APP}/daxx

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./daxx"]
