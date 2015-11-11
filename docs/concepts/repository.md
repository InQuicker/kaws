# Repositories

A kaws repository is just a Git repository with a particular file structure.
A repository is necessary to perform all other kaws commands.
When you run `kaws init`, the new repository will look like this:
```
.
├── .gitignore
├── clusters
├── pubkeys
└── terraform
    ├── main.tf
```

The top-level directories are as follows:

* clusters – Stores digital certificates, OpenPGP-encrypted private keys, state files, and configuration files for each Kubernetes cluster created with kaws.
* pubkeys – Stores OpenPGP public keys for each member of your team that will need access to the Kubernetes API for any of your clusters.
* terraform – Stores Terraform configuration files that define the Kubernetes cluster topology.

The bulk of the Terraform configuration itself lives in a Terraform module in a subdirectory of the kaws repository on GitHub.
The entrypoint file, `terraform/main.tf` declares this module dependency and passes in all necessary variables to make `kaws cluster` commands work.
The Terraform module will be locked to the version corresponding with the tagged release of kaws that generated the repository.
If you want to customize the Terraform module used to deploy Kubernetes, you can provide a custom Terraform module source using the `--terraform-source` option of `kaws init`.
This option is also useful when working on the kaws code itself, since you can point it at the repository on your local disk.

All files that are not ignored via the `.gitignore` file are intended to be checked into Git.
