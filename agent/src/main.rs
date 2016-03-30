extern crate etcd;
extern crate clap;
extern crate rusoto;
extern crate rustc_serialize;

mod agent;
mod decryption;

use std::process::exit;

use clap::{App, AppSettings, Arg, SubCommand};

use agent::Agent;

fn main() {
    let mut failed = false;

    match execute_cli() {
        Ok(success) => {
            if let Some(message) = success {
                println!("{}", message);
            }
        }
        Err(error) => {
            println!("Error:\n{}", error);

            failed = true;
        }
    }

    if failed {
        exit(1);
    }
}

fn execute_cli() -> Result<Option<String>, String> {
    let app_matches = App::new("kaws-agent")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Server-side agent for kaws that manages cluster security credentials")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the agent")
                .arg(
                    Arg::with_name("role")
                        .short("r")
                        .long("role")
                        .takes_value(true)
                        .possible_values(&["master", "node"])
                        .required(true)
                        .help("The role of the server the agent will be running on")
                )
        )
        .get_matches();

    match app_matches.subcommand() {
        ("run", Some(matches)) => Agent::new(matches).run(),
        _ => {
            println!("{}", app_matches.usage());

            Ok(None)
        }
    }
}
