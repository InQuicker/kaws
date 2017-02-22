TAG = 0.6.0

all: dist

.PHONY: clean
clean:
	rm -f dist/*
	cargo clean

target/release/kaws:
	cargo build --release

target/x86_64-unknown-linux-musl/release/kaws:
	docker run \
		--rm \
		-v $(PWD):/volume \
		-v $(HOME)/.cargo/git:/root/.cargo/git \
		-v $(HOME)/.cargo/registry:/root/.cargo/registry \
		-w /volume \
		-t \
		clux/muslrust \
		cargo build --release

.PHONY: docker-build
docker-build:
	docker build -t inquicker/kaws -t inquicker/kaws:$(TAG) .

dist: clean dist/sha256sums.txt.sig docker-build

dist/kaws-$(TAG)-darwin.tar.gz: target/release/kaws
	tar -c -C target/release -zvf dist/kaws-$(TAG)-darwin.tar.gz kaws

dist/kaws-$(TAG)-linux.tar.gz: target/x86_64-unknown-linux-musl/release/kaws
	tar -c -C target/x86_64-unknown-linux-musl/release -zvf dist/kaws-$(TAG)-linux.tar.gz kaws

dist/sha256sums.txt: dist/kaws-$(TAG)-darwin.tar.gz dist/kaws-$(TAG)-linux.tar.gz
	cd dist && shasum -a 256 * > sha256sums.txt

dist/sha256sums.txt.sig: dist/sha256sums.txt
	cd dist && gpg2 --detach-sign sha256sums.txt
