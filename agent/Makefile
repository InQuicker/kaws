all: cargo-build docker-build

cargo-build:
	docker run --rm -v ${PWD}:/source -v ${HOME}/.cargo:/root/.cargo jimmycuadra/rust cargo build --release --target x86_64-unknown-linux-gnu

docker-build:
	docker build -t inquicker/kaws-agent .
