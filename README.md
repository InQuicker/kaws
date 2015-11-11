# kaws

**kaws** is a tool for deploying multiple [Kubernetes](http://kubernetes.io/) clusters.
It ties together several other tools to make Kubernetes deployment easy, repeatable, and secure.

kaws is not intended to support every possible deployment scenario for a Kubernetes cluster.
It takes a highly opinionated approach using specific software, services, and conventions.
Specifically, kaws creates Kubernetes clusters in [AWS](https://aws.amazon.com/) using [CoreOS](https://coreos.com/) servers.

## Status

kaws has not yet reached version 1.0, and is not recommended for production usage until it has. In accordanace with [Semantic Versioning](http://semver.org/), while kaws is < 1.0, backwards incompatible changes may occur. Major expected changes include moving from OpenPGP to AWS Key Management Service, using AWS autoscaling groups, and changing Kubernetes's container runtime to rkt. See the [issues](https://github.com/InQuicker/kaws/issues) for details.

**kaws has not been reviewed by security professionals.** For information about the threat model of kaws, see the [security](docs/concepts/security.md) document.

## Synopsis

```
USAGE:
	kaws [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

SUBCOMMANDS:
    admin      Commands for managing cluster administrators
    cluster    Commands for managing a cluster's infrastructure
    help       Prints this message
    init       Initializes a new repository for managing Kubernetes clusters
    key        Commands for managing the OpenPGP keys
```

## Goals

* Define infrastructure as code for predictability and repeatability
* Produce secure, highly available Kubernetes clusters with DNS built in
* Generate and distribute Kubernetes API access credentials securely
* Avoid shell scripting as much as possible

## Supported platforms

At this time, kaws has only been developed for and tested on OS X. Support for Linux is planned.

## Installing dependencies

kaws requires the following other programs to be available on your system:

* [Terraform](https://terraform.io/)
* [OpenSSL](https://www.openssl.org/)
* [GNU Privacy Guard](https://www.gnupg.org/)
* [kubectl](http://kubernetes.io/)

### OS X

All the dependencies can be installed with [Homebrew](http://brew.sh/):

```
brew install terraform openssl gpg2 kubernetes-cli
brew link --force openssl
```

The `brew link` command is necessary because the openssl formula is "keg only," in turn because OS X ships with an older version already installed.
Running the link command makes the newer one installed by Homebrew the default.

You may wish to install [GPGTools](https://gpgtools.org/) instead of installing gpg2 with Homebrew.
GPGTools will install the gpg2 program needed for this project.
It also includes a nice GUI for creating and managing keys and a plugin for Apple Mail for signing and encrypting email.

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
