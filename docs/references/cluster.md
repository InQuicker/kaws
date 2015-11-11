# kaws cluster

`kaws cluster groups commands for managing a Kubernetes cluster's infrastructure.

## Synopsis

```
USAGE:
	kaws cluster [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

SUBCOMMANDS:
    apply        Applies the Terraform plan to the target cluster
    destroy      Destroys resources defined by the Terraform plan for the target cluster
    help         Prints this message
    init         Initializes all the configuration files for a new cluster
    plan         Displays the Terraform plan for the target cluster
    reencrypt    Re-encrypts the cluster's SSL keys, allowing decryption by new administrators
```

## Subcommands

### apply

`kaws cluster apply` applies the Terraform plan to the target cluster.

```
USAGE:
	kaws cluster apply [FLAGS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    cluster    The cluster whose plan should be applied
```

This command is a simple wrapper around `terraform apply` that points at the right Terraform configuration and state files for the target cluster.

### destroy

`kaws cluster destroy` destroys resources defined by the Terraform plan for the target cluster.

```
USAGE:
	kaws cluster destroy [FLAGS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    cluster    The cluster to destroy
```

This command is a simple wrapper around `terraform destroy` that points at the right Terraform configuration and state files for the target cluster.

### init

`kaws cluster init` initializes all the configuration files for a new cluster.

```
USAGE:
	kaws cluster init [FLAGS] [OPTIONS] <cluster> --domain <domain> --uid <uid> --ami <ami> --instance-size <size> --ssh-key <ssh-key> --kubernetes-version <k8s-version> --zone-id <zone-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

OPTIONS:
    -a, --ami <ami>                           EC2 AMI ID to use for all CoreOS instances, e.g. "ami-1234"
    -d, --domain <domain>                     The base domain name for the cluster, e.g. "example.com"
    -v, --kubernetes-version <k8s-version>    Version of Kubernetes to use, e.g. "1.0.0"
    -r, --recipient <recipient>...            OpenPGP UID for an additional key allowed to decrypt the CA, master, and node keys
    -s, --instance-size <size>                EC2 instance size to use for all instances, e.g. "m3.medium"
    -k, --ssh-key <ssh-key>                   Name of the SSH key in AWS for accessing EC2 instances, e.g. "alice"
    -u, --uid <uid>                           OpenPGP UID for the encryption key
    -z, --zone-id <zone-id>                   Zone ID of the Route 53 hosted zone

ARGS:
    cluster    The name of the cluster to create, e.g. "production"
```

This command creates the directory `clusters/CLUSTER` in your kaws repository with the Terraform variable file, Terraform state file, and public key infrastructure files necessary to create the cluster.
it takes a number of options which are required for the initial configuration.
Of particular note are:

* `--domain`: The base domain for the cluster. An AWS Route 53 hosted zone must exist for this domain. The subdomain "kubernetes" will be created to provide access to the Kubernetes API.
* `--uid`: The OpenPGP UID of the administrator running the command. This administrator will be able to decrypt the cluster's certificate authority, master, and node private keys.
* `--recipient`: Additional OpenPGP UIDs for administrators that should also be able to decrypt the cluster's private keys. Optional and can be specified multiple times.
* `--zone-id`: The zone ID from AWS Route 53 for the domain specified with `--domain`.

A version of CoreOS that includes `kubelet` (773.1.0 or greater) is required.
As of November 11, 2015, this means the beta or alpha channels.
See the [CoreOS release notes](https://coreos.com/releases/) for the most up to date information.
Find the latest EC2 AMI ID for the release channel you choose on [Running CoreOS on EC2](https://coreos.com/os/docs/latest/booting-on-ec2.html).

### plan

`kaws cluster plan` displays the Terraform plan for the target cluster.

```
USAGE:
	kaws cluster plan [FLAGS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    cluster    The cluster whose plan should be displayed
```

This command is a simple wrapper around `terraform plan` that points at the right Terraform configuration and state files for the target cluster.

### reencrypt

`kaws cluster reencrypt` re-encrypts the cluster's SSL keys, allowing decryption by new administrators.

```
USAGE:
	kaws cluster reencrypt [FLAGS] [OPTIONS] <cluster> --uid <uid>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

OPTIONS:
    -r, --recipient <recipient>...    OpenPGP UID for a key that will be allowed to decrypt the re-encrypted keys
    -u, --uid <uid>                   OpenPGP UID for the decryption key

ARGS:
    cluster    The cluster whose keys should be re-encrypted
```

This command is used to change which administrators are able to decrypt the certificate authority, master, and node private keys for the target cluster.
When the keys are first created with `kaws cluster init`, the administrator running the command, plus any additional administrators that were specified can decrypt the keys.
If you want to allow new administrators to decrypt them, or prevent currently allowed administrators from decrypting them, this command allows you to specify a new list.
The administrator running the command must be able to decrypt the keys, as the command will decrypt and rencrypt them.
