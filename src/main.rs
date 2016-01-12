extern crate ansi_term;
extern crate clap;
extern crate rusoto;

mod admin;
mod cli;
mod cluster;
mod encryption;
mod error;
mod key;
mod log;
mod process;
mod repository;
mod terraform;

use std::process::exit;

use ansi_term::Colour::Green;

use admin::Admin;
use cluster::Cluster;
use error::Result;
use key::Key;
use repository::Repository;
use terraform::Terraform;

fn main() {
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

fn execute_cli() -> Result {
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
            ("reencrypt", Some(matches)) => Cluster::new(matches).reencrypt(),
            _ => {
                println!("{}", cluster_matches.usage());

                Ok(None)
            },
        },
        ("init", Some(matches)) => Repository::new(matches).create(),
        ("key", Some(key_matches)) => match key_matches.subcommand() {
            ("export", Some(matches)) => Key::new(matches).export(),
            _ => {
                println!("{}", key_matches.usage());

                Ok(None)
            },
        },
        _ => {
            println!("{}", app_matches.usage());

            Ok(None)
        },
    }
}
