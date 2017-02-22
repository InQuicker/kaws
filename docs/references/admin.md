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
    help       Prints this message or the help message of the given subcommand(s)
    install    Configures kubectl for a new cluster and administrator
    sign       Signs an administrator's certificate signing request, creating a new client certificate
```

## Subcommands

### create

`kaws admin create` generates a private key and certificate signing request for a new adminsitrator.

```
USAGE:
    kaws admin create [OPTIONS] <cluster> <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --group <group>...    A Kubernetes groups this user belongs to; this option can be specified more than once

ARGS:
    <cluster>    The cluster the new administrator should be able to access
    <name>       The new administrator's name
```

Creates the following files:

* clusters/CLUSTER/NAME-key.pem: The admin's unencrypted private key
* clusters/CLUSTER/NAME-csr.pem: The admin's certificate signing request

Generated files are only valid for the specified cluster.
The private key should not be checked into Git.

### install

`kaws admin install` configures `kubectl` for a new cluster/administrator.

```
USAGE:
    kaws admin install [FLAGS] <cluster> <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <cluster>    The cluster to configure
    <name>       The new administrator's name
```

The following files are expected by this command:

* clusters/CLUSTER/k8s-ca.pem: The k8s CA certificate
* clusters/CLUSTER/NAME.pem: The admin's client certificate
* clusters/CLUSTER/NAME-key.pem: The admin's unencrypted private key

### sign

`kaws admin sign` signs an administrator's certificate signing request, creating a new client certificate.

```
USAGE:
    kaws admin install <cluster> <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <cluster>    The cluster to configure
    <name>       The name of the administrator whose credentials are being installed
```

The following files are expected by this command:

* clusters/CLUSTER/k8s-ca.pem: The CA certificate
* clusters/CLUSTER/k8s-ca-key-encrypted.base64: The KMS-encrypted CA private key
* clusters/CLUSTER/NAME-csr.pem: The requesting administrator's CSR
