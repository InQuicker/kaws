use clap::{App, AppSettings, Arg, SubCommand};

pub fn app<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    App::new("kaws")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Deploys Kubernetes clusters using AWS, CoreOS, GnuPG, and Terraform")
        .after_help("Start by creating a new repository with the `init` command.\n")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .global(true)
                .help("Outputs additional information to the standard output")
        )
        .subcommand(admin())
        .subcommand(cluster())
        .subcommand(init())
        .subcommand(key())
}

fn admin<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("admin")
        .about("Commands for managing cluster administrators")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(admin_create())
        .subcommand(admin_install())
        .subcommand(admin_sign())
}

fn admin_create<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("create")
        .about("Generates a private key and certificate signing request for a new administrator")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster the new administrator should be able to access")
        )
        .arg(
            Arg::with_name("uid")
                .index(2)
                .required(true)
                .help("OpenPGP UID of the new administrator")
        )
        .after_help(
            "Creates the following files:\n\n\
            * clusters/CLUSTER/UID-key.pem.asc: The OpenPGP-encrypted private key\n\
            * clusters/CLUSTER/UID.csr: The certificate signing request\n\n\
            The user specified by UID must have the OpenPGP public and secret keys in their\n\
            local keyring. Generated files are only valid for the specified cluster.\n"
        )
}

fn admin_install<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("install")
        .about("Configures kubectl for a new cluster and administrator")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster to configure")
        )
        .arg(
            Arg::with_name("uid")
                .index(2)
                .required(true)
                .help("OpenPGP UID of the administrator")
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("The base domain name for the cluster, e.g. \"example.com\"")
        )
        .after_help(
            "The following files are expected by this command:\n\n\
            * clusters/CLUSTER/ca.pem: The CA certificate\n\
            * clusters/CLUSTER/UID.pem: The client certificate\n\
            * clusters/CLUSTER/UID-key.pem.asc: The OpenPGP encrypted private key\n\n\
            The user specified by UID must have the OpenPGP secret key in their local\n\
            keyring.\n"
        )
}

fn admin_sign<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("sign")
        .about("Signs an administrator's certificate signing request, creating a new client certificate")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The name of the cluster the certificate will be valid for")
        )
        .arg(
            Arg::with_name("recipient")
                .index(2)
                .required(true)
                .help("OpenPGP UID of the requesting administrator")
        )
        .after_help(
            "The following files are expected by this command:\n\n\
            * clusters/CLUSTER/ca.pem: The CA certificate\n\
            * clusters/CLUSTER/ca-key.pem: The CA private key\n\
            * clusters/CLUSTER/RECIPIENT.csr: The requesting administrator's CSR\n\n\
            The user running the command must have an OpenPGP secret key allowed to\n\
            decrypt the CA private key in their local keyring.\n"
        )
}

fn cluster<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("cluster")
        .about("Commands for managing a cluster's infrastructure")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(cluster_apply())
        .subcommand(cluster_destroy())
        .subcommand(cluster_init())
        .subcommand(cluster_plan())
        .subcommand(cluster_reencrypt())
}

fn cluster_apply<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("apply")
        .about("Applies the Terraform plan to the target cluster")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster whose plan should be applied")
        )
}

fn cluster_destroy<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("destroy")
        .about("Destroys resources defined by the Terraform plan for the target cluster")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster to destroy")
        )
}

fn cluster_init<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("init")
        .about("Initializes all the configuration files for a new cluster")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The name of the cluster to create, e.g. \"production\"")
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("The base domain name for the cluster, e.g. \"example.com\"")
        )
        .arg(
            Arg::with_name("uid")
                .short("u")
                .long("uid")
                .takes_value(true)
                .required(true)
                .help("OpenPGP UID for the encryption key")
        )
        .arg(
            Arg::with_name("recipient")
                .short("r")
                .long("recipient")
                .takes_value(true)
                .multiple(true)
                .help("OpenPGP UID for an additional key allowed to decrypt the CA, master, and node keys")
        )
        .arg(
            Arg::with_name("ami")
                .short("a")
                .long("ami")
                .takes_value(true)
                .required(true)
                .help("EC2 AMI ID to use for all CoreOS instances, e.g. \"ami-1234\"")
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("instance-size")
                .takes_value(true)
                .required(true)
                .help("EC2 instance size to use for all instances, e.g. \"m3.medium\"")
        )
        .arg(
            Arg::with_name("ssh-key")
                .short("k")
                .long("ssh-key")
                .takes_value(true)
                .required(true)
                .help("Name of the SSH key in AWS for accessing EC2 instances, e.g. \"alice\"")
        )
        .arg(
            Arg::with_name("k8s-version")
                .short("v")
                .long("kubernetes-version")
                .takes_value(true)
                .required(true)
                .help("Version of Kubernetes to use, e.g. \"1.0.0\"")
        )
        .arg(
            Arg::with_name("zone-id")
                .short("z")
                .long("zone-id")
                .takes_value(true)
                .required(true)
                .help("Zone ID of the Route 53 hosted zone")
        )
}

fn cluster_plan<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("plan")
        .about("Displays the Terraform plan for the target cluster")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster whose plan should be displayed")
        )
}

fn cluster_reencrypt<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("reencrypt")
        .about("Re-encrypts the cluster's SSL keys, allowing decryption by new administrators")
        .arg(
            Arg::with_name("cluster")
                .index(1)
                .required(true)
                .help("The cluster whose keys should be re-encrypted")
        )
        .arg(
            Arg::with_name("uid")
                .short("u")
                .long("uid")
                .takes_value(true)
                .required(true)
                .help("OpenPGP UID for the decryption key")
        )
        .arg(
            Arg::with_name("recipient")
                .short("r")
                .long("recipient")
                .takes_value(true)
                .multiple(true)
                .help("OpenPGP UID for a key that will be allowed to decrypt the re-encrypted keys")
        )
}
fn init<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("init")
        .about("Initializes a new repository for managing Kubernetes clusters")
        .arg(
            Arg::with_name("name")
                .index(1)
                .required(true)
                .help("The name of the repository to create, e.g. \"example-company-infrastructure\"")
        )
        .arg(
            Arg::with_name("terraform-source")
                .short("t")
                .long("terraform-source")
                .takes_value(true)
                .help("Custom source value for the Terraform module to use")
        )
}

fn key<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("key")
        .about("Commands for managing the OpenPGP keys")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(key_export())
}


fn key_export<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    SubCommand::with_name("export")
        .about("Exports an OpenPGP public key from the local keyring into the pubkeys directory")
        .arg(
            Arg::with_name("uid")
                .index(1)
                .required(true)
                .help("The OpenPGP UID of the key to export")
        )
}
