ARG RUSTUP_HOME=/usr/local/rustup
ARG CARGO_HOME=/usr/local/cargo
ARG NIGHTLY=2022-06-20
ARG TARGET=x86_64-unknown-linux-gnu

FROM debian:buster as rustup
ENV PATH=$CARGO_HOME:$PATH
RUN # apt-get update && \
	# apt-get install -y curl && \
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
	rustup install nightly-$NIGHTLY-$TARGET

FROM debian:buster-slim
COPY --from=rustup $CARGO_HOME $CARGO_HOME
COPY --from=rustup $RUSTUP_HOME $RUSTUP_HOME
