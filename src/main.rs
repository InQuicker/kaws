extern crate ansi_term;
extern crate env_logger;
extern crate clap;
extern crate rusoto;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

macro_rules! log_wrap {
    ($m:expr, $b:block) => {
        debug!("{}...", $m);
        $b
        debug!("...done.");
    }
}

mod admin;
mod aws;
mod cli;
mod cluster;
mod encryption;
mod error;
mod process;
mod repository;
mod terraform;

use std::process::exit;

use ansi_term::Colour::Green;

use admin::Admin;
use cluster::Cluster;
use error::KawsResult;
use repository::Repository;
use terraform::Terraform;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");

    let mut failed = false;

    match execute_cli() {
        Ok(success) => {
            if let Some(message) = success {
                println!("{}", Green.paint(message.to_string()));
            }
        },
        Err(error) => {
            println!("Error:\n{}", error);

            failed = true;
        },
    }

    if failed {
        exit(1);
    }
}

fn execute_cli() -> KawsResult {
    let app_matches = cli::app().get_matches();

    match app_matches.subcommand() {
        ("admin", Some(admin_matches)) => match admin_matches.subcommand() {
            ("create", Some(matches)) => Admin::new(matches).create(),
            ("install", Some(matches)) => Admin::new(matches).install(),
            ("sign", Some(matches)) => Admin::new(matches).sign(),
            _ => {
                println!("{}", admin_matches.usage());

                Ok(None)
            },
        },
        ("cluster", Some(cluster_matches)) => match cluster_matches.subcommand() {
            ("apply", Some(matches)) => Terraform::new(matches).apply(),
            ("destroy", Some(matches)) => Terraform::new(matches).destroy(),
            ("init", Some(matches)) => Cluster::new(matches).init(),
            ("plan", Some(matches)) => Terraform::new(matches).plan(),
            _ => {
                println!("{}", cluster_matches.usage());

                Ok(None)
            },
        },
        ("init", Some(matches)) => Repository::new(matches).create(),
        _ => {
            println!("{}", app_matches.usage());

            Ok(None)
        },
    }
}
