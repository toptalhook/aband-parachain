#This is the build stage for Polkadot. Here we create the binary in a temporary image .
FROM docker.io/druaken/ci-linux:latest as builder

WORKDIR /testnet

COPY substrate-parachain-PoS-template /testnet/substrate-parachain-PoS-template
COPY polkadot /testnet/polkadot

RUN ln -s /root/.cargo/bin/cargo /usr/local/bin/cargo && \
	cd substrate-parachain-PoS-template && \
	make submodule && \
	cargo build --locked --release && \
	cd ../polkadot && \
	cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Polkadot binary. "

FROM docker.io/druaken/ubuntu_aband:latest

COPY --from=builder /testnet/substrate-parachain-PoS-template/target/release/aband /usr/local/bin/
COPY --from=builder /testnet/polkadot/target/release/polkadot /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /testnet polkadot && \
	mkdir -p /data /testnet/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /testnet/.local/share/data

COPY --from=builder /testnet/substrate-parachain-PoS-template/scripts/config.json /data

EXPOSE 9966 30666 9955 30555 9944 30333 9999 31300
VOLUME ["/data"]
ENTRYPOINT ["polkadot-launch","/data/config.json"]
