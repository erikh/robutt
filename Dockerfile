FROM rust

RUN mkdir -p /code
COPY . /code

WORKDIR /code
RUN cargo install --path .

RUN mkdir -p /mnt

WORKDIR /mnt
CMD robutt config.yml
