# Cluster workflows

## Creating a new cluster

The process of creating a cluster involves the following steps:

1.  Create an AWS account and an API access key if you haven't already. If you're using IAM and want to restrict the scope of the access key, it will need to be able to perform operations only on VPC, EC2, ELB, Route 53, and IAM resources. The access key ID and secret access key are loaded from the environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`, if present, falling back to the `~/.aws/credentials` file.
2.  Create a hosted zone for the domain for your cluster in Route 53, e.g. example.com.
3.  Create the initial files for the new cluster with the [kaws cluster init](../references/cluster.md#init) command.
4.  Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be created.
5.  Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, creating the cluster. This will take several minutes.

You can then move on to [creating an administrator](admin.md) and using the cluster.

## Destroying a cluster

To destroy a cluster, simply run the [kaws cluster destroy](../references/cluster.md#destroy) command, then remove the cluster's directory from the kaws repository.

## Adding a VPN connection to a cluster

If applications in your cluster require a VPN connection to access resources on other networks, you can specify this using kaws.

1. Create a VPN connection in AWS, either using the AWS console, API, or by defining it in Terraform configuration in your kaws repository.
2. Add the ID of the VPN connection's Virtual Private Gateway (in the form "vgw-1234abc") to the `kaws_propagating_vgws` list in the file at `clusters/CLUSTER_NAME/terraform.tfvars`.
3. Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be modified.
4. Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, modifying the cluster.

Routes from the VPN connection will now be propgated into the cluster's public route table.
