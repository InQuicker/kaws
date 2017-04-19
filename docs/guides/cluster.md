# Cluster workflows

## Creating a new cluster

The process of creating a cluster involves the following steps:

1.  Create an AWS account and an API access key if you haven't already. If you're using IAM and want to restrict the scope of the access key, it will need to be able to perform operations only on VPC, EC2, ELB, Route 53, and IAM resources. The access key ID and secret access key are loaded from the environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`, if present, falling back to the `~/.aws/credentials` file.
2.  Create a hosted zone for the domain for your cluster in Route 53, e.g. example.com.
3.  Create the initial files for the new cluster with the [kaws cluster init](../references/cluster.md#init) command.
4.  Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be created.
5.  Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, creating the cluster. This will take several minutes.
6.  [Create an administrator](admin.md) who belongs to the `system:masters` group.
7.  Run `kubectly apply -f rbac.yml` where `rbac.yml` is a file with the following contents:

    ``` yaml
    kind: "RoleBinding"
    apiVersion: "rbac.authorization.k8s.io/v1beta1"
    metadata:
      name: "kube-system-serviceaccount-cluster-admin"
      namespace: "kube-system"
    subjects:
      - kind: "ServiceAccount"
        name: "default"
        namespace: "kube-system"
    roleRef:
      kind: "ClusterRole"
      apiGroup: "rbac.authorization.k8s.io"
      name: "cluster-admin"
    ```

    This will grant full cluster access to all of the Kubernetes core components (as well as any other pods running in the kube-system namespace with the default service account.)
    In the future, kaws will use more granular access control for the core components.

    Once this role binding has been created, the Kubernetes nodes will be able to register themselves with the Kubernetes API, and will then show up in the output of `kubectl get nodes`.
    The other kubernetes components will soon appear in the output of `kubectl get pods -n kube-system`.

## Destroying a cluster

To destroy a cluster, simply run the [kaws cluster destroy](../references/cluster.md#destroy) command, then remove the cluster's directory from the kaws repository.

## Adding a VPN connection to a cluster

If applications in your cluster require a VPN connection to access resources on other networks, you can specify this using kaws.

1. Create a VPN connection in AWS, either using the AWS console, API, or by defining it in Terraform configuration in your kaws repository.
2. Add the ID of the VPN connection's Virtual Private Gateway (in the form "vgw-1234abc") to the `kaws_propagating_vgws` list in the file at `clusters/CLUSTER_NAME/terraform.tfvars`.
3. Optional: Use the [kaws cluster plan](../references/cluster.md#plan) command to display the Terraform plan and see what AWS resources will be modified.
4. Run [kaws cluster apply](../references/cluster.md#apply) to apply the Terraform plan, modifying the cluster.

Routes from the VPN connection will now be propgated into the cluster's public route table.
