# kaws-agent

**kaws-agent** is the server-side component of **kaws**. It watches etcd for changes to the key-value pairs storing certificates and private keys for Kubernetes, pulls them out, uses AWS KMS to decrypt them, writes them to disk in the correct place, and restarts the necessary systemd units.

## Legal

kaws-agent is released under the MIT license. See `LICENSE` in the parent project, kaws, for details.
