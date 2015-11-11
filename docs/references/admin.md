# kaws admin

`kaws admin` groups commands for managing cluster administrators.

## Synopsis

```
USAGE:
	kaws admin [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

SUBCOMMANDS:
    create     Generates a private key and certificate signing request for a new administrator
    help       Prints this message
    install    Configures kubectl for a new cluster and administrator
    sign       Signs an administrator's certificate signing request, creating a new client certificate
```

## Subcommands

### create

`kaws admin create` generates an private key and certificate signing request for a new adminsitrator.

```
USAGE:
	kaws admin create [FLAGS] <cluster> <uid>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    cluster    The cluster the new administrator should be able to access
    uid        OpenPGP UID of the new administrator
```

This command creates the following files:

* clusters/CLUSTER/UID-key.pem.asc: The OpenPGP-encrypted private key
* clusters/CLUSTER/UID.csr: The certificate signing request

The user specified by UID must have the OpenPGP public and secret keys in their local keyring.

### install

`kaws admin install` configures `kubectl` for a new cluster/administrator.

```
USAGE:
	kaws admin install [FLAGS] <cluster> <uid> --domain <domain>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

OPTIONS:
    -d, --domain <domain>    The base domain name for the cluster, e.g. "example.com"

ARGS:
    cluster    The cluster to configure
    uid        OpenPGP UID of the administrator
```

The following files are expected by this command:

* clusters/CLUSTER/ca.pem: The CA certificate
* clusters/CLUSTER/UID.pem: The client certificate
* clusters/CLUSTER/UID-key.pem.asc: The OpenPGP encrypted private key

The user specified by UID must have the OpenPGP secret key in their local keyring.

### sign

`kaws admin sign` signs an administrator's certificate signing request, creating a new client certificate.

```
USAGE:
	kaws admin sign [FLAGS] <cluster> <recipient>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    cluster      The name of the cluster the certificate will be valid for
    recipient    OpenPGP UID of the requesting administrator
```

The following files are expected by this command:

* clusters/CLUSTER/ca.pem: The CA certificate
* clusters/CLUSTER/ca-key.pem: The CA private key
* clusters/CLUSTER/RECIPIENT.csr: The requesting administrator's CSR

The user running the command must have an OpenPGP secret key allowed to decrypt the CA private key in their local keyring.
