use clap::ArgMatches;

use error::Result;
use process::execute_child_process;

pub struct Key<'a> {
    uid: &'a str,
}


impl<'a> Key<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        Key {
            uid: matches.value_of("uid").expect("clap should have required uid"),
        }
    }

    pub fn export(&self) -> Result {
        try!(execute_child_process("gpg2", &[
            "--output",
            &format!("pubkeys/{}.asc", self.uid),
            "--armor",
            "--export",
            self.uid,
        ]));

        Ok(Some(format!(
            "OpenPGP public key exported to pubkeys/{}.asc! Check this file into Git.",
            self.uid,
        )))
    }
}
