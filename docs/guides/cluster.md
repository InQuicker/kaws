# Cluster workflows

## Creating a new cluster

The process of creating a cluster involves the following steps:

1. Create an AWS account and an API access key if you haven't already. If you're using IAM and want to restrict the scope of the access key, it will need to be able to perform operations only on VPC, EC2, ELB, Route 53, and IAM resources. The access key ID and secret access key are loaded from the environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`, if present, falling back to the `~/.aws/credentials` file.
2. Create a hosted zone for the domain for your cluster in Route 53, e.g. example.com.
3. Create the initial files for the new cluster with the [kaws cluster init](../references/cluster.md#init) command.
4. Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be created.
5. Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, creating the cluster. This will take several minutes.

## Destroying a cluster

To destroy a cluster, simply run the [kaws cluster destroy](../references/cluster.md#destroy) command, then remove the cluster's directory from the kaws repository.
