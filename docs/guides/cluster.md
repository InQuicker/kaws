# Cluster workflows

## Creating a new cluster

The process of creating a cluster involves the following steps:

1. Create an AWS account and an API access key if you haven't already. If you're using IAM and want to restrict the scope of the access key, it will need to be able to perform operations only on VPC, EC2, ELB, Route 53, and IAM resources. The access key ID and secret access key values must be present in the environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`, respectively.
2. Create a hosted zone for the domain for your cluster in Route 53, e.g. example.com.
3. Use GNU Privacy Guard to create an OpenPGP key pair for yourself if you don't already have one.
4. Add your OpenPGP public key to the `pubkeys` directory using [kaws key export](../references/key.md#export).
5. Ensure that any other administrators you want to be [primary administrators](../concepts/admin.md) of the cluster have also added their public keys to the kaws repository.
6. Create the initial files for the new cluster with the [kaws cluster init](../references/cluster.md#init) command. Remember to supply the `--recipient` option for any additional administrators who should be designated as primary.
7. Ensure the SSH private key you specified when initializing the cluster is loaded by `ssh-agent`: `ssh-add /path/to/private/key`.
8. Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be created.
9. Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, creating the cluster. This will take several minutes.

## Destroying a cluster

To destroy a cluster, simply run the [kaws cluster destroy](../references/cluster.md#destroy) command, then remove the cluster's directory from the kaws repository.
