FROM debian:buster as rustup
ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	NIGHTLY=2022-06-20 \
	TARGET=x86_64-unknown-linux-gnu
ENV PATH=$CARGO_HOME/bin:$PATH
RUN apt-get update && \
	apt-get install -y gcc libsdl2-dev curl clang
RUN	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
	| sh -s -- --profile minimal -y --default-toolchain nightly-$NIGHTLY && \
	rustup install nightly-$NIGHTLY-$TARGET
