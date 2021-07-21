FROM rust

RUN mkdir -p /code
COPY . /code

WORKDIR /code
RUN cargo install --path .

COPY config.yml /config.yml

CMD robutt config.yml
