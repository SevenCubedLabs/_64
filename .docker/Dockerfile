FROM debian:buster as rustup
ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	NIGHTLY=2022-06-20 \
	TARGET=x86_64-unknown-linux-gnu
ENV PATH=$CARGO_HOME/bin:$PATH
RUN echo $PATH && \
 	apt-get update && \
	apt-get install -y curl && \
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y && \
	rustup install nightly-$NIGHTLY-$TARGET

FROM debian:buster-slim
COPY --from=rustup $CARGO_HOME $CARGO_HOME
COPY --from=rustup $RUSTUP_HOME $RUSTUP_HOME
