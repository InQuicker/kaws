# Administrator workflows

## Giving an administrator API access to a cluster

In order for an adminstrator to use `kubectl` or have any sort of programmatic access to a Kubernetes cluster, they must have a client certificate for the cluster.

1. Create an AWS KMS customer master key for the new administrator. This key should be readable only by the new administrator. The same customer master key can be shared across clusters for the same administrator. However, this customer master key is distinct from the one used to encrypt the cluster's certificate authority, Kubernetes master, and Kubernetes node private keys. It is used only to encrypt the personal private key of the new administrator.
2. Have the new administrator run [kaws admin create](../references/admin.md#create) to generate an encrypted private key and certificate signing request and commit them to the repository.
3. Any [primary administrator](admin.md#primary-administrators) runs [kaws admin sign](../references/admin.md#sign) to generate the new administrator's client certificate and commit it to the repository.
4. Have the new administrator run [kaws admin install](../references/admin.md#install) to automatically configure their local copy of `kubectl` to authenticate with the Kubernetes API for that cluster.

At this point, `kubectl` can be used as usual.
Steps 2-4 are repeated for each cluster an administrator should have access to.
When generating credentials for a [primary administrator](admin.md#primary-administrators), all of the above steps are performed by the same person.
