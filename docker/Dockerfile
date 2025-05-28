FROM ubuntu:18.04

RUN apt-get update && apt-get install -y gcc-multilib curl

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add i686-unknown-linux-gnu

COPY docker-entrypoint.sh /
CMD ["/docker-entrypoint.sh"]