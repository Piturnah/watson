FROM rust

RUN apt update
RUN apt install -y libpq-dev

WORKDIR /app
COPY . /app

RUN cargo install --no-default-features --path .
RUN rm -rf /app/*

EXPOSE 3000

CMD ["watson-server"]
