# OpenPGP

OpenPGP (specifically GNU Privacy Guard, a prevalent free implementation of OpenPGP) is used for encrypting private keys that are stored in the [kaws repository](repository.md).
This allows administrators' access credentials for the Kubernetes API, as well as the certificate and key used to serve the API itself, from being seen by anyone who shouldn't be able to.

The `pubkeys` directory in a kaws repository contains OpenPGP public keys for any users that will be administering a Kubernetes cluster.
These keys are shared across all clusters.
The public keys in the `pubkeys` directory are stored in the ASCII armor format (.asc) and are named by their UIDs.
All public keys in the `pubkeys` directory will be automatically imported to the local OpenPGP keyring (which is managed by GNU Privacy Guard) at the beginning of any kaws command that involves OpenPGP.

## UID format

GNU Privacy Guard accepts quite a number of different values for identifying a key in the local keyring, including key ID, fingerprint, exact and substring matches on the user's name, email address, etc.
kaws assumes consistent usage of the same value for any given OpenPGP key, so it is recommended to use a consistent naming scheme for all key UIDs.
In particular, the username portion of a user's email address is recommended.
For example, if Example Company, Inc. has three Kubernetes administrators with email addresses alice@example.com, bob@example.com, and carol@example.com, the values *alice*, *bob*, and *carol* should always be used when specifying one of their OpenPGP UIDs in a kaws command.
Specifically, this value is used for:

* The name of the public key file in the `pubkeys` directory of the kaws repository
* The name of the client certiticate, certificate signing request, and private keys of administrators
* The subject name of administrators' client certificates
* The arguments supplied to `gpg2` when encryption operations are performed

Currently, kaws does not enforce consistent usage of UID value. This may change in the future to prevent errors and security risks.
