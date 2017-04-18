# Administrators

An administrator, as is used by the `kaws admin` commands, is a user who has access to the Kubernetes API for a cluster.
Credentials for the API are needed in order to administer the cluster with the `kubectl` command line tool.
kaws configures Kubernetes clusters to use SSL client certificates for authentication.
Each administrator's client certificate and certificate signing request are stored in the clusters directory in the kaws repository.
The private keys for each administrator are not checked into Git and should be kept private.
A separate SSL client certificate and private key is needed per administrator per cluster.

## Primary administrators

There is also a special kind of administrator (referred to throughout the documentation as "primary adminstrator") that can decrypt the private keys for the certificate authority, Kubernetes API server, and Kubernetes node components (kubelet and kube-proxy).
Anyone who has access to the AWS KMS customer master key used to encrypt these private keys is a primary administrator.
Generally, you'll also want the primary administrator to have full access to the Kubernetes API.
This is done by including the group `system:masters` in the administrator's certificate signing request.
Any user of the Kubernetes API with this group is bound to the `cluster-admin` cluster role through the [default RBAC resources](https://kubernetes.io/docs/admin/authorization/rbac/#default-roles-and-role-bindings) in Kubernetes 1.6+.
