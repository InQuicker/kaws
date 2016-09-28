FROM scratch

ENTRYPOINT ["/kaws"]

COPY target/x86_64-unknown-linux-musl/release/kaws /kaws

