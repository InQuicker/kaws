# Administrators

An administrator, as is used by the `kaws admin` commands, is a user who has access to the Kubernetes API for a cluster, and possibly access to decrypt the private keys used for the cluster's public key infrastructure.
Credentials for the API are needed in order to administer the cluster with the `kubectl` command line tool.
kaws configures Kubernetes clusters to use SSL client certificates for authentication.
Each administrator's client certificate, certificate signing request, and private key are stored in the clusters directory in the kaws repository.
The private keys for each administrator are encrypted using OpenPGP and can only be decrypted by the appropriate administrator.
A separate SSL client certificate and private key is needed per administrator per cluster.

## Primary administrators

There is also a special kind of administrator (referred to throughout the documentation as "primary adminstrator") that can decrypt the private keys for the certificate authority, Kubernetes API server, and Kubernetes node components (kubelet and kube-proxy).
The administrator who initially creates a cluster is always a primary administrator for that cluster.
The primary administrator can choose for other administrators to be designated as primary at cluster creation time, as well.
If an administrator is to become a primary after the initial cluster creation, this can be done at any time using the `kaws cluster reencrypt` command.
