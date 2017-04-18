# Administrator workflows

## Giving an administrator API access to a cluster

In order for an adminstrator to use `kubectl` or have any sort of programmatic access to a Kubernetes cluster, they must have a client certificate for the cluster.

1. Have the new administrator run [kaws admin create](../references/admin.md#create) to generate a private key and certificate signing request. Commit the CSR to the Git repository. The private key is ignored by Git via the .gitignore file.
If the user being created is a [primary administrator](admin.md#primary-administrators), use the `--group` option to include the group `system:masters` in the certificate signing request.
Any user of the Kubernetes API with this group is bound to the `cluster-admin` cluster role through the [default RBAC resources](https://kubernetes.io/docs/admin/authorization/rbac/#default-roles-and-role-bindings) in Kubernetes 1.6+.
2. Any [primary administrator](admin.md#primary-administrators) runs [kaws admin sign](../references/admin.md#sign) to generate the new administrator's client certificate and commit it to the repository.
3. Have the new administrator run [kaws admin install](../references/admin.md#install) to automatically configure their local copy of `kubectl` to authenticate with the Kubernetes API for that cluster.

At this point, `kubectl` can be used as usual.
This process is repeated for each cluster an administrator should have access to.
When generating credentials for a [primary administrator](admin.md#primary-administrators), all of the above steps are performed by the same person.
