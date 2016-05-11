# Repositories

A kaws repository is just a Git repository with a particular file structure.
A repository is necessary to perform all other kaws commands.
When you run `kaws init`, the new repository will look like this:
```
.
├── .gitignore
├── clusters
└── terraform
    └── kaws
        └── main.tf
```

The top-level directories are as follows:

* clusters – Stores digital certificates, encrypted private keys, state files, and configuration files for each Kubernetes cluster created with kaws.
* terraform - Stores any custom terraform configuration the user desires.
* terraform/kaws – Stores Terraform configuration files that define the Kubernetes cluster topology.

The bulk of the Terraform configuration itself lives in a Terraform module in a subdirectory of the kaws repository on GitHub.
The entrypoint file, `terraform/kaws/main.tf` declares this module dependency and passes in all necessary variables to make `kaws cluster` commands work.
The Terraform module will be locked to the version corresponding with the tagged release of kaws that generated the repository.
If you want to customize the Terraform module used to deploy Kubernetes, you can provide a custom Terraform module source using the `--terraform-source` option of `kaws init`.
This option is also useful when working on the kaws code itself, since you can point it at the repository on your local disk.

All files that are not ignored via the `.gitignore` files are intended to be checked into Git.
