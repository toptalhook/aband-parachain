# Forked from: https://github.com/paritytech/polkadot

FROM docker.io/druaken/ci-linux:latest as builder

WORKDIR /aband/substrate-parachain-PoS-template
COPY substrate-parachain-PoS-template /aband/substrate-parachain-PoS-template

RUN ln -s /root/.cargo/bin/cargo /usr/local/bin/cargo && \
	make submodule && \
	cargo build --locked --release
	
FROM docker.io/library/ubuntu:20.04

LABEL description="A PoS-based parachain template." \
	io.aband.image.type="builder" \
	io.aband.image.authors="hi@aband.io" \
	io.aband.image.vendor="Aband DAO" \
	io.aband.image.description="A PoS-based parachain template." \
	io.aband.image.source="https://github.com/Aband-Network/substrate-parachain-PoS-template/blob/${VCS_REF}/scripts/aband_builder.Dockerfile" \
	io.aband.image.documentation="https://github.com/Aband-Network/substrate-parachain-PoS-template"

COPY --from=builder /aband/substrate-parachain-PoS-template/target/release/aband /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /aband aband && \
	mkdir -p /data /aband/.local/share && \
	chown -R aband:aband /data && \
	ln -s /data /aband/.local/share/aband 
# unclutter and minimize the attack surface
	#rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	#/usr/local/bin/aband --version

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["aband","--","--chain","kusama"]
