use std::ffi::OsStr;
use std::fmt::Display;
use std::process::Command;

use error::{KawsError, KawsResult};

pub fn execute_child_process<S: AsRef<OsStr> + Display>(program: S, args: &[S]) -> KawsResult {
    let mut command = Command::new(&program);
    command.args(args);
    let output = command.output()?;

    if !output.status.success() {
        return Err(
            KawsError::with_std_streams(
                format!("Execution of `{:?}` failed.", command),
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
        );
    }

    Ok(None)
}
