# kaws cluster

`kaws cluster` groups commands for managing a Kubernetes cluster's infrastructure.

## Synopsis

```
USAGE:
    kaws cluster [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apply           Applies the Terraform plan to the target cluster
    destroy         Destroys resources defined by the Terraform plan for the target cluster
    generate-pki    Generates public key infrastructure for a cluster
    help            Prints this message or the help of the given subcommand(s)
    init            Initializes all the configuration files for a new cluster
    output          Displays the Terraform outputs for the target cluster
    plan            Displays the Terraform plan for the target cluster
    refresh         Refreshes the Terraform state for the target cluster
```

## Subcommands

### apply

`kaws cluster apply` applies the Terraform plan to the target cluster.

```
USAGE:
    kaws cluster apply [FLAGS] [OPTIONS] <cluster> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-path <aws-credentials-path>          Path to the AWS credentials file, defaults to ~/.aws/credentials
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    <cluster>    The cluster whose plan should be applied
```

This command is a simple wrapper around `terraform apply` that points at the right Terraform configuration and state files for the target cluster.
Any arguments following a literal `--` will be passed directly as options to `terraform apply`.

### destroy

`kaws cluster destroy` destroys resources defined by the Terraform plan for the target cluster.

```
USAGE:
    kaws cluster destroy [FLAGS] [OPTIONS] <cluster> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-path <aws-credentials-path>          Path to the AWS credentials file, defaults to ~/.aws/credentials
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    <cluster>    The cluster to destroy
```

This command is a simple wrapper around `terraform destroy` that points at the right Terraform configuration and state files for the target cluster.
Any arguments following a literal `--` will be passed directly as options to `terraform destroy`.

### generate-pki

`kaws cluster generate-pki` generates public key infrastructure for a cluster.

```
USAGE:
    kaws cluster generate-pki [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    all           Generates all necessary public key infrastructure for a new cluster
    etcd          Generates public key infrastructure for etcd's client API
    etcd-peer     Generates public key infrastructure for etcd's peer API
    help          Prints this message or the help of the given subcommand(s)
    kubernetes    Generates public key infrastructure for Kubernetes
```

These commands are used to generate (or regenerate) X.509 certificates required by etcd and the Kubernetes system components.
Certificates are required for etcd's client API ("ca", "client", and "server"), etcd's peer API ("ca", "peer"), Kubernetes ("ca"), Kubernetes control plane components ("masters"), and kubelet on the Kubernetes nodes ("nodes").

kaws's Terraform configuration will execute the "all" subcommand during initial cluster creation to set all of this up.
The certificates generated in this process will eventually expire.
Before they do, you can generate new ones using the various subcommands, and then re-running `kaws cluster apply`.

### init

`kaws cluster init` initializes all the configuration files for a new cluster.

```
USAGE:
    kaws cluster init <cluster> --ami <ami> --availability-zone <availability-zone> --aws-account-id <aws-account-id> --cidr <cidr> --domain <domain> --iam-user <iam-user>... --kubernetes-version <k8s-version> --masters-max-size <masters-max-size> --masters-min-size <masters-min-size> --nodes-max-size <nodes-max-size> --nodes-min-size <nodes-min-size> --region <region> --instance-size <size> --ssh-key <ssh-key>... --zone-id <zone-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --ami <ami>                                EC2 AMI ID to use for all CoreOS instances, e.g. "ami-1234"
        --availability-zone <availability-zone>    Availability Zone for etcd instances and EBS volumes, e.g. "us-east-1a"
    -A, --aws-account-id <aws-account-id>          The numeric ID of the AWS account, e.g. "123456789012"
    -C, --cidr <cidr>                              IPv4 network range of the subnet where Kubernetes nodes will run, e.g. "10.0.2.0/24"
    -d, --domain <domain>                          The base domain name for the cluster, e.g. "example.com"
    -i, --iam-user <iam-user>...                   An IAM user name who will have access to cluster PKI secrets, e.g. "alice"; this option can be specified more than once
    -v, --kubernetes-version <k8s-version>         Version of Kubernetes to use, e.g. "1.0.0"
        --masters-max-size <masters-max-size>      The maximum number of EC2 instances the Kubernetes masters may autoscale to
        --masters-min-size <masters-min-size>      The minimum number of EC2 instances the Kubernetes masters may autoscale to
        --nodes-max-size <nodes-max-size>          The maximum number of EC2 instances the Kubernetes nodes may autoscale to
        --nodes-min-size <nodes-min-size>          The minimum number of EC2 instances the Kubernetes nodes may autoscale to
    -r, --region <region>                          AWS Region to create the resources in, e.g. "us-east-1"
    -s, --instance-size <size>                     EC2 instance size to use for all instances, e.g. "m3.medium"
    -K, --ssh-key <ssh-key>...                     SSH public key to add to ~/.ssh/authorized_keys on each server; this option can be specified more than once
    -z, --zone-id <zone-id>                        Route 53 hosted zone ID

ARGS:
    <cluster>    The name of the cluster to create, e.g. "production"
```

This command creates the directory `clusters/CLUSTER` in your kaws repository with the Terraform variable file, Terraform state file, and public key infrastructure files necessary to create the cluster.
it takes a number of options which are required for the initial configuration.
Of particular note are:

* `--domain`: The base domain for the cluster. An AWS Route 53 hosted zone must exist for this domain.
  The subdomain "kubernetes" will be created to provide access to the Kubernetes API and "bastion" as the SSH entrypoint to the cluster.
* `--kms-key`: The AWS KMS customer master key to use for encrypting the cluster's SSL private keys.
* `--zone-id`: The zone ID from AWS Route 53 for the domain specified with `--domain`.

Find the latest EC2 AMI ID for the release channel you choose on [Running CoreOS on EC2](https://coreos.com/os/docs/latest/booting-on-ec2.html).

### output

`kaws cluster output` displays the Terraform outputs for the target cluster.

```
USAGE:
    kaws cluster output [FLAGS] <cluster> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <cluster>    The cluster whose plan should be displayed
    [output]     The name of an individual output to display
```

This command is a simple wrapper around `terraform apply` that points at the right Terraform configuration and state files for the target cluster.
It can print all outputs, or a single named output, if the name of the output is supplied as an additional parameter.
This command is used internally by the `kaws admin` commands, but may be useful to users as well.

### plan

`kaws cluster plan` displays the Terraform plan for the target cluster.

```
USAGE:
    kaws cluster plan [FLAGS] [OPTIONS] <cluster> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-path <aws-credentials-path>          Path to the AWS credentials file, defaults to ~/.aws/credentials
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    <cluster>    The cluster whose plan should be displayed
```

This command is a simple wrapper around `terraform plan` that points at the right Terraform configuration and state files for the target cluster.
Any arguments following a literal `--` will be passed directly as options to `terraform plan`.

### refresh

`kaws cluster refresh` refreshes the Terraform state for the target cluster.

```
USAGE:
    kaws cluster refresh [OPTIONS] <cluster> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aws-credentials-path <aws-credentials-path>          Path to the AWS credentials file, defaults to ~/.aws/credentials
        --aws-credentials-profile <aws-credentials-profile>    Name of the AWS credentials profile to use, defaults to "default"

ARGS:
    <cluster>    The cluster whose plan should be displayed
```

This command is a simple wrapper around `terraform refresh` that points at the right Terraform configuration and state files for the target cluster.
Any arguments following a literal `--` will be passed directly as options to `terraform refresh`.
