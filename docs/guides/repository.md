# Repository workflows

## Creating a kaws repository

A [kaws repository](../concepts/repository.md) is required for all other kaws commands. Start by creating one.

Create a kaws repository by running:

```
kaws init NAME
```

Where `NAME` is the name you'd like to give it, such as *my-company-infrastructure*.

Advanced: If you need to use a custom Terraform module for any reason, you can specify its source with the `--terraform-source` option.

Inside the directory created, run `git init` and then commit all the files in it to the Git repository. The Git repository should be shared in a location accessible by all Kubernetes administrators on your team.

Next: [Cluster workflows](cluster.md)
