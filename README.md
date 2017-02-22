# kaws

**kaws** is a tool for creating and managing [Kubernetes](http://kubernetes.io/) clusters on [AWS](https://aws.amazon.com/) using [Terraform](https://www.terraform.io/).
It ties together several other tools to make Kubernetes deployment easy, repeatable, and secure.

kaws is not intended to support every possible deployment scenario for Kubernetes clusters.
It follows a specific approach used by InQuicker, involving specific software, services, and conventions.
Specifically, kaws creates Kubernetes clusters in AWS using [CoreOS](https://coreos.com/) servers, all managed by declarative configuration files with Terraform.

## Status

kaws has not yet reached version 1.0, and is not recommended for production usage until it has.
In accordance with [Semantic Versioning](http://semver.org/), while kaws is < 1.0, backwards incompatible changes may occur.
See the [issues](https://github.com/InQuicker/kaws/issues) for details.

The CoreOS and Kubernetes teams have plans for Kubernetes to be "self-hosting" in the future.
If and when this vision is complete, there won't need to be such tight coupling between infrastructure provisioning and Kubernetes tooling.
At that point, kaws (and likely kube-aws, see the next section) may be retired.
See [Self-Hosted Kubernetes](https://coreos.com/blog/self-hosted-kubernetes.html) and [bootkube](https://github.com/kubernetes-incubator/bootkube) for more information.

**kaws has not been reviewed by security professionals.**
For information about the threat model of kaws, see the [security](docs/concepts/security.md) document.

## Similar tools

When kaws was originally created, none of the following tools existed, which is why we chose to develop it.
Since then, these tools have been released publicly.
They are each developed by larger teams and with broader use cases in mind.
kaws is still used by InQuicker because it works the best with our particular configuration.
However, you should consider these other tools instead, if they fit your needs:

* [kube-aws](https://github.com/coreos/kube-aws) from CoreOS.
  This is the most similar to kaws, but does not use Terraform.
* [kops](https://github.com/kubernetes/kops) from Kubernetes.
  Supports exporting configuration to Terraform format, but is not built around a Terraform-based infrastructure.
  Does not default to CoreOS servers.
  Unclear what the relationship is with kubeadm, since both projects are under the Kubernetes organization.
* [kubeadm](https://kubernetes.io/docs/getting-started-guides/kubeadm/) from Kubernetes.
  An alpha-status tool included with the Kubernetes distribution itself.
  Does not handle the infrastructure Kubernetes is running on.

## Synopsis

```
USAGE:
    kaws [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    admin      Commands for managing cluster administrators
    cluster    Commands for managing a cluster's infrastructure
    help       Prints this message or the help message of the given subcommand(s)
    init       Initializes a new repository for managing Kubernetes clusters
```

Start by creating a new repository with the `init` command.

## Goals

* Define infrastructure as code for predictability and repeatability
* Produce secure, highly available Kubernetes clusters
* Generate and distribute Kubernetes API access credentials securely
* Avoid shell scripting as much as possible

## Supported platforms

At this time, kaws has only been developed for and tested on macOS.

## Installing dependencies

kaws requires the following other programs to be available on your system:

* [Terraform](https://terraform.io/), version 0.8 or greater
* [cfssl](https://github.com/cloudflare/cfssl)
* [kubectl](http://kubernetes.io/), version 1.5 or greater

### macOS

All the dependencies can be installed with [Homebrew](http://brew.sh/):

```
brew install terraform cfssl kubernetes-cli
```

## Installing kaws

Once all the required dependencies are installed on your system, you can install kaws.

### Precompiled binaries

Signed precompiled binaries for tagged version numbers are available for download on the [releases](https://github.com/InQuicker/kaws/releases) page.

### Building from source

1. Install the appropriate version of [Rust](https://www.rust-lang.org/) for your system.
2. Run `git clone git@github.com:InQuicker/kaws.git`.
3. Inside the freshly cloned repository, run `cargo build --release`.
4. Copy the binary from `target/release/kaws` to a directory in your PATH, such as `/usr/local/bin`.

## Documentation

Detailed documentation is available in the [docs](docs) directory. A good place to start is the [overview](docs/overview.md).

## Development

To package the current release for distribution, update `TAG` in the Makefile and then run `make`.
Release artifacts will be written to the `dist` directory.
Your GPG secret key will be required to sign `sha256sums.txt`.

Docker images for `inquicker/kaws` and `inquicker/kaws:$TAG` will be created, but you must push them manually.
`cargo publish` must be run manually to release to crates.io.

## Legal

kaws is released under the MIT license. See `LICENSE` for details.
