# kaws

**kaws** is a tool for creating and managing [Kubernetes](http://kubernetes.io/) clusters on [AWS](https://aws.amazon.com/) using [Terraform](https://www.terraform.io/).
It ties together several other tools to make Kubernetes deployment easy, repeatable, and secure.

kaws is not intended to support every possible deployment scenario for Kubernetes clusters.
It follows a specific approach used by InQuicker, involving specific software, services, and conventions.
Specifically, kaws creates Kubernetes clusters in AWS using [CoreOS](https://coreos.com/) servers, all managed by declarative configuration files with Terraform.

## Status

kaws has not yet reached version 1.0, and is not recommended for production usage until it has.
In accordanace with [Semantic Versioning](http://semver.org/), while kaws is < 1.0, backwards incompatible changes may occur.
Major expected changes include using rkt as the container runtime for Kubernetes.
See the [issues](https://github.com/InQuicker/kaws/issues) for details.

**kaws has not been reviewed by security professionals.**
For information about the threat model of kaws, see the [security](docs/concepts/security.md) document.

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

At this time, kaws has only been developed for and tested on OS X. Support for Linux is planned.

## Installing dependencies

kaws requires the following other programs to be available on your system:

* [Terraform](https://terraform.io/), version 0.7 or greater
* [OpenSSL](https://www.openssl.org/)
* [kubectl](http://kubernetes.io/), version 1.3.0 or greater

### OS X

All the dependencies can be installed with [Homebrew](http://brew.sh/):

```
brew install terraform openssl kubernetes-cli
brew link --force openssl
```

The `brew link` command is necessary because the openssl formula is "keg only," in turn because OS X ships with an older version already installed.
Running the link command makes the newer one installed by Homebrew the default.

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

## Legal

kaws is released under the MIT license. See `LICENSE` for details.
