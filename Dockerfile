FROM scorpil/rust:nightly
MAINTAINER Mike Engel <mike@mike-engel.com>

ENV APP_DIR=/app/locale \
    ROCKET_ENV=prod

RUN mkdir -p APP_DIR

WORKDIR ${APP_DIR}

COPY . ${APP_DIR}/

RUN cargo update \
    && cargo build --release

EXPOSE 8000

CMD ["./target/release/locale"]
