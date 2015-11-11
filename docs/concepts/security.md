# Security

Security should be a chief concern for anyone running services online.
This document provides an overview of the approach kaws uses for securing Kubernetes clusters and the various attack surfaces that users should be aware of.
There are three primary classes of resources that need to be secured in the kaws system: the Kubernetes components, the AWS EC2 servers running the cluster, and the SSL and SSH keys used to authenticate.

## Kubernetes

Kubernetes itself is a complex system with several independent services that work in tandem to provide their benefits.
The entire security profile of Kubernetes is out of scope for this document.
The parts relevant to kaws are authentication and authorization between cluster administrators and the Kubernetes API and between the Kubernetes components themselves.
kaws configures the cluster to use SSL client certificates for authentication in both cases.
The Kubernetes master servers have a copy of the master private key and the node servers have a copy of the node private key. At present, all non-master Kubernetes components share the same client certificate and key.
Each individual administrator has their own client certificate and key.
All of these certificates are signed by a certificate authority unique to the cluster.
At this time, kaws itself does not perform any configuration related to authorization.
If different administrators should have different levels of access to the Kubernetes API, this must be handled by the [primary administrators](admin.md#primary-administrators).

### Threat model

* Compromised Kubernetes SSL credentials would give access to everything Kubernetes can see and control.

## AWS resources

All servers in a kaws-built Kubernetes cluster exist in a public subnet of their VPC.
This is necessary because each server needs to be accessed from outside the VPC: the bastion server must accept external SSH, the Kubernetes masters must serve the Kubernetes API, and the Kubernetes nodes must serve any web applications the administrators run on them.
At this time, kaws does not use a VPN or private subnet with NAT, though this may change in future versions.
Security groups are configured for each AWS resource to allow only the neccessary incoming traffic from the external Internet.
The bastion server is the only server that accepts incoming SSH connections on port 22.
The Kubernetes master servers (and load balancer) accept incoming connections on 443 only.
The Kubernetes nodes accept incoming connections on both 80 and 443.
Port 80 is open so that applications can redirect to a secure version on port 443 using HSTS headers.
kaws does not enforce this in any way, however.
SSH access is not managed at all by kaws, other than allowing the user to specify the name of the key to be used when launching EC2 instances.
If additional SSH access is required, it must be handled by the primary administrators.

### Threat model

* Compromised SSH keys would give complete control of all data in the cluster.
* Applications exposed to the external Internet are vulnerable to attacks that are beyond the scope of kaws, but could potentially result in anything up to and including unrestricted access to all data in the cluster.
* Unencrypted communications to Kubernetes nodes on port 80 are vulnerable to a man in the middle attack, but can be mitigated by redirecting all HTTP requests to HTTPS and using HSTS.
* SSL private keys are currently stored unencrypted on the EBS-backed EC2 instances themselves. Anyone with access to the servers has access to the private keys, which compromises the entire cluster.
* The security of AWS itself is assumed, but any vulnerability in AWS's various services would apply to the Kubernetes cluster as well.

# Public key infrastructure

One of the benefits of kaws is that it automates the creation of the public key infrastructure used to secure communications between Kubernetes components and administrators.
kaws uses OpenSSL to generate certificates and keys and GNU Privacy Guard to keep all private keys encrypted at rest.
For visiblity and convenience, kaws keeps the public keys of all cluster administrators in the `pubkeys` directory of the kaws repository.
However, kaws does not currently use its own OpenPGP keyring.
It merely imports the public keys in the directory into the administrator's local keychain.
This is likely to change in the future.
kaws relies on the administrators to use a consistent form for commands that accept OpenPGP UID values.
These values are used as the names of the files in the `pubkeys` and `clusters/CLUSTER` directories.
This means that if the value used to identify a key is not unique, it is possible to accidentally use the wrong key, which would give access to the wrong person to decrypt private keys.
The `kaws cluster reencrypt` command does not change the private keys themselves, so it cannot be used as a revocation mechanism when removing an OpenPGP key from the list of keys allowed to perform decryption.
In the future, the consistency of this naming scheme will likely be enforced by kaws.

### Threat model

* Compromised OpenPGP private keys would give an attacker access to the PKI private keys, and potentially the entire Kubernetes API.
* Vulnerabilities in OpenSSL and GnuPG themselves affect any resources that rely on them for security.
* Certificate signing requests generated by administrators are not currently verified against the administrator's OpenPGP key, instead relying on the administrator's access to the kaws Git repository for authenticity. If an attacker is able to get a commit into the repository with a bad OpenPGP public key, the Kubernetes API should be assumed compromised.
* Inconsistent or careless usage of values for OpenPGP UIDs may give an undesired key access to decrypt PKI private keys. This will likely be improved in the future.
* Keys present in an administrator's local keyring that are not in the `pubkeys` directory of the kaws repository could result in undesired PKI private key access. This will likely be improved in the future.
