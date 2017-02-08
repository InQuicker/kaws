use std::ffi::OsStr;
use std::fmt::Display;
use std::process::Command;
use std::str::from_utf8;

use ansi_term::Colour::Red;

use error::{KawsError, KawsResult};

pub fn execute_child_process<S: AsRef<OsStr> + Display>(program: S, args: &[S]) -> KawsResult {
    let mut command = Command::new(&program);
    command.args(args);
    let output = command.output()?;

    if !output.status.success() {
        let error_message = &format!("
Execution of `{:?}` failed! The output of the program are detailed below:

stdout:
{}

stderr:
{}
", command, from_utf8(&output.stdout)?, from_utf8(&output.stderr)?);

        return Err(KawsError::new(format!("{}", Red.paint(error_message.to_string()))));
    }

    Ok(None)
}
