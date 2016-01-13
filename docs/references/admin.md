# kaws admin

`kaws admin` groups commands for managing cluster administrators.

## Synopsis

```
USAGE:
	kaws admin [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

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
	kaws admin create [FLAGS] <cluster> <name> --kms-key <kms-key>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --kms-key <kms-key>    KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"

ARGS:
    cluster    The cluster the new administrator should be able to access
    name       The new administrator's name
```

Creates the following files:

* clusters/CLUSTER/NAME-key.pem.encrypted: The KMS-encrypted private key
* clusters/CLUSTER/NAME.csr: The certificate signing request

Generated files are only valid for the specified cluster.

### install

`kaws admin install` configures `kubectl` for a new cluster/administrator.

```
USAGE:
	kaws admin install [FLAGS] <cluster> <name> --kms-key <kms-key> --domain <domain>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>      The base domain name for the cluster, e.g. "example.com"
    -k, --kms-key <kms-key>    KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"

ARGS:
    cluster    The cluster to configure
    name       The new administrator's name
```

The following files are expected by this command:

* clusters/CLUSTER/ca.pem: The CA certificate
* clusters/CLUSTER/NAME.pem: The client certificate
* clusters/CLUSTER/NAME-key.pem.encrypted: The KMS-encrypted private key

### sign

`kaws admin sign` signs an administrator's certificate signing request, creating a new client certificate.

```
USAGE:
	kaws admin sign [FLAGS] <cluster> <name> --kms-key <kms-key>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --kms-key <kms-key>    KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"

ARGS:
    cluster    The name of the cluster the certificate will be valid for
    name       The new administrator's name
```

The following files are expected by this command:

* clusters/CLUSTER/ca.pem: The CA certificate
* clusters/CLUSTER/ca-key.pem.encrypted: The KMS-encrypted CA private key
* clusters/CLUSTER/NAME.csr: The requesting administrator's CSR
