# kaws cluster

`kaws cluster groups commands for managing a Kubernetes cluster's infrastructure.

## Synopsis

```
USAGE:
	kaws cluster [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apply        Applies the Terraform plan to the target cluster
    destroy      Destroys resources defined by the Terraform plan for the target cluster
    help         Prints this message
    init         Initializes all the configuration files for a new cluster
    plan         Displays the Terraform plan for the target cluster
    reencrypt    Re-encrypts the cluster's SSL keys using a new AWS KMS customer master key
```

## Subcommands

### apply

`kaws cluster apply` applies the Terraform plan to the target cluster.

```
USAGE:
	kaws cluster apply [FLAGS] [OPTIONS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    cluster    The cluster whose plan should be applied
```

This command is a simple wrapper around `terraform apply` that points at the right Terraform configuration and state files for the target cluster.

### destroy

`kaws cluster destroy` destroys resources defined by the Terraform plan for the target cluster.

```
USAGE:
	kaws cluster destroy [FLAGS] [OPTIONS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    cluster    The cluster to destroy
```

This command is a simple wrapper around `terraform destroy` that points at the right Terraform configuration and state files for the target cluster.

### init

`kaws cluster init` initializes all the configuration files for a new cluster.

```
USAGE:
	kaws cluster init [FLAGS] <cluster> --domain <domain> --kms-key <kms-key> --ami <ami> --instance-size <size> --ssh-key <ssh-key> --kubernetes-version <k8s-version> --zone-id <zone-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --ami <ami>                           EC2 AMI ID to use for all CoreOS instances, e.g. "ami-1234"
    -d, --domain <domain>                     The base domain name for the cluster, e.g. "example.com"
    -v, --kubernetes-version <k8s-version>    Version of Kubernetes to use, e.g. "1.0.0"
    -k, --kms-key <kms-key>                   KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"
    -s, --instance-size <size>                EC2 instance size to use for all instances, e.g. "m3.medium"
    -K, --ssh-key <ssh-key>                   Name of the SSH key in AWS for accessing EC2 instances, e.g. "alice"
    -z, --zone-id <zone-id>                   Route 53 hosted zone ID

ARGS:
    cluster    The name of the cluster to create, e.g. "production"
```

This command creates the directory `clusters/CLUSTER` in your kaws repository with the Terraform variable file, Terraform state file, and public key infrastructure files necessary to create the cluster.
it takes a number of options which are required for the initial configuration.
Of particular note are:

* `--domain`: The base domain for the cluster. An AWS Route 53 hosted zone must exist for this domain. The subdomain "kubernetes" will be created to provide access to the Kubernetes API.
* `--kms-key`: The AWS KMS customer master key to use for encrypting the cluster's SSL private keys.
* `--zone-id`: The zone ID from AWS Route 53 for the domain specified with `--domain`.

Find the latest EC2 AMI ID for the release channel you choose on [Running CoreOS on EC2](https://coreos.com/os/docs/latest/booting-on-ec2.html).

### plan

`kaws cluster plan` displays the Terraform plan for the target cluster.

```
USAGE:
	kaws cluster plan [FLAGS] [OPTIONS] <cluster>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-path <aws-credentials-path>          Path to AWS credentials file, defaults to "~/.aws/credentials"
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    cluster    The cluster whose plan should be displayed
```

This command is a simple wrapper around `terraform plan` that points at the right Terraform configuration and state files for the target cluster.

### reencrypt

`kaws cluster reencrypt` re-encrypts the cluster's SSL keys using a new AWS KMS customer master key.

```
USAGE:
	kaws cluster reencrypt [FLAGS] <cluster> --current-key <current-key> --new-key <new-key>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --current-key <current-key>    Current KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"
        --new-key <new-key>            New KMS customer master key ID, e.g. "12345678-1234-1234-1234-123456789012"

ARGS:
    cluster    The cluster whose keys should be re-encrypted
```
