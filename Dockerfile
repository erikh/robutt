FROM rust

RUN mkdir -p /code
COPY . /code

WORKDIR /code
RUN cargo install --path .

COPY config.yml /config.yml
RUN mkdir -p /mnt

WORKDIR /mnt
CMD robutt config.yml
