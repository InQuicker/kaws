# Overview

**kaws** is a command line tool for installing and managing Kubernetes clusters.
kaws operates on a *kaws repository* which is just a Git repository with a particular file structure.
The repository contains a Terraform module which defines the Kubernetes cluster toplogy, digital certificates and keys for the Kubernetes API, and OpenPGP public keys to facilitate the generation and transfer of Kubernetes API credentials for administrators.

The basic workflow for using kaws is:

1. Create a kaws repository.
2. Create a new Kubernetes cluster.
3. Generate and distribute Kubernetes API credentials to administrators.
4. Operate the Kubernetes cluster normally using `kubectl`.

Repeat steps 2-3 for each additional Kubernetes cluster desired. Usually a staging cluster and production cluster will be desired at a minimum.

If you want to get started right away, begin with the [repository workflows](guides/repository.md) guide. Otherwise, detailed documentation in three forms is available below.

## Concepts

* [Administrators](concepts/admin.md)
* [Clusters](concepts/cluster.md)
* [OpenPGP](concepts/key.md)
* [Repositories](concepts/repository.md)
* [Terraform](concepts/terraform.md)
* [Security](concepts/security.md)

## Guides

* [Administrator workflows](guides/admin.md)
* [Cluster workflows](guides/cluster.md)
* [Repository workflows](guides/repository.md)

## References

* [kaws admin](references/admin.md)
* [kaws cluster](references/cluster.md)
* [kaws init](references/init.md)
* [kaws key](references/key.md)
