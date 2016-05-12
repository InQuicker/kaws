# Repositories

A kaws repository is just a Git repository with a particular file structure.
A repository is necessary to perform all other kaws commands.
When you run `kaws init`, the new repository will look like this:
```
.
├── .gitignore
├── clusters
└── terraform
    └── kaws.tf
```

The top-level directories are as follows:

* clusters – Stores digital certificates, encrypted private keys, state files, and configuration files for each Kubernetes cluster created with kaws.
* terraform – Stores Terraform configuration files for each cluster.

The Terraform module for Kubernetes itself lives in  a subdirectory of kaws's own Git repository on GitHub.
The entry point file, `terraform/kaws.tf` imports this module and passes in all necessary variables to make `kaws cluster` commands work.
The Terraform module will be locked to the version corresponding with the tagged release of kaws that generated the repository.
If you want to customize the Terraform module used to deploy Kubernetes, you can provide a custom Terraform module source using the `--terraform-source` option of `kaws init`.
This option is also useful when working on the kaws code itself, since you can point it at the repository on your local disk.

Additional custom Terraform resources the user desires can be defined in additional `.tf` files in the terraform directory, alongside `kaws.tf`.
If you have resources that are specific to a certain cluster, consider moving that cluster to a separate kaws repository.
Each cluster within a single kaws repository is intended to have identical infrastructure, to encourage staging and production environments being identical.

All files that are not ignored via the `.gitignore` files are intended to be checked into Git.
