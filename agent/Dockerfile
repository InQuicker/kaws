FROM scratch

ENTRYPOINT ["/kaws-agent"]

COPY etc /etc/ # Can be deleted once https://github.com/coreos/rkt/pull/2599 is in CoreOS stable
COPY target/x86_64-unknown-linux-musl/release/kaws-agent /kaws-agent
