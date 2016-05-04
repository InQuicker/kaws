use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use clap::ArgMatches;
use etcd::Client;

use decryption::Decryptor;

pub struct Agent {
    decryptor: Decryptor,
    etcd: Client,
    role: Role,
}

#[derive(Clone, Copy)]
enum Role {
    Master,
    Node,
}

impl Agent {
    pub fn new<'a>(matches: &'a ArgMatches) -> Result<Self, String> {
        Ok(Agent {
            decryptor: try!(Decryptor::new(
                matches.value_of("region").expect("clap should have required region")
            )),
            etcd: Client::new(&["http://127.0.0.1:2379"]).expect("Failed to create etcd client"),
            role: matches.value_of("role").expect("clap should have required role").parse().expect(
              "clap should have required a valid value for role"
            )
        })
    }

    pub fn run(mut self) -> Result<Option<String>, String> {
        println!("kaws-agent is starting...");
        println!("Checking current modifiedIndex of /kaws key...");
        let mut modified_index = match self.etcd.get("/kaws", false, false, false) {
            Ok(key_space_info) => match key_space_info.node.unwrap().modified_index {
                Some(new_index) => new_index,
                None => return Err("WARNING: etcd node for /kaws had no modified index".to_owned()),
            },
            Err(errors) => return Err(errors[0].to_string()),
        };

        if let Err(error) = self.refresh() {
            println!("ERROR: {}", error);
        }

        loop {
            println!("Watching /kaws key...");
            match self.etcd.watch("/kaws", Some(modified_index), true) {
                Ok(key_space_info) => {
                    if let Err(error) = self.refresh() {
                        println!("WARNING: {}", error);
                    }

                    match key_space_info.node.unwrap().modified_index {
                        Some(new_index) => modified_index = new_index,
                        None => println!("WARNING: etcd node for watched key had no modified index"),
                    }
                }
                Err(errors) => println!("ERROR: {}", errors[0]),
            }
        }
    }

    // private

    fn refresh(&mut self) -> Result<Option<String>, String> {
        println!("Refreshing files from data in etcd...");
        try!(self.refresh_item("/kaws/pki/ca.pem", "/etc/kubernetes/ssl/ca.pem", false));

        match &self.role {
            &Role::Master => {
                try!(self.refresh_item(
                    "/kaws/pki/master.pem",
                    "/etc/kubernetes/ssl/master.pem",
                    false,
                ));
                try!(self.refresh_item(
                    "/kaws/pki/master-key.pem",
                    "/etc/kubernetes/ssl/master-key.pem",
                    true,
                ));
            }
            &Role::Node => {
                try!(self.refresh_item(
                    "/kaws/pki/node.pem",
                    "/etc/kubernetes/ssl/node.pem",
                    false,
                ));
                try!(self.refresh_item(
                    "/kaws/pki/node-key.pem",
                    "/etc/kubernetes/ssl/node-key.pem",
                    true,
                ));
            }
        }

        Ok(None)
    }

    fn refresh_item(&mut self, key: &str, file_path: &str, encrypted: bool)
    -> Result<Option<String>, String> {
        println!("Refreshing file {} from etcd key {}", file_path, key);
        let key_space_info = match self.etcd.get(key, false, false, false) {
            Ok(key_space_info) => key_space_info,
            Err(errors) => return Err(errors[0].to_string()),
        };

        let value = match key_space_info.node.unwrap().value {
            Some(value) => if encrypted {
                match self.decryptor.decrypt(&value) {
                    Ok(plaintext) => plaintext,
                    Err(error) => return Err(error),
                }
            } else {
                value
            },
            None => return Err(format!("etcd key {} had no value", key)),
        };

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => return Err(error.to_string()),
        };

        match file.write_all(value.as_bytes()) {
            Ok(_) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(Role::Master),
            "node" => Ok(Role::Node),
            role => Err(format!("Unknown server role for agent: {}", role)),
        }
    }
}
