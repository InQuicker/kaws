use std::process::{Command, Stdio};

use error::{KawsError, KawsResult};

pub fn ensure_dependencies() -> KawsResult {
    ensure_openssl().and(ensure_kubectl()).and(ensure_terraform())
}

fn ensure_openssl() -> KawsResult {
    let installed = match Command::new("openssl")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status() {
            Ok(status) => status.success(),
            Err(_) => false,
    };

    if installed {
        Ok(None)
    } else {
        Err(KawsError::new("openssl must be installed".to_string()))
    }
}

fn ensure_kubectl() -> KawsResult {
    let installed = match Command::new("kubectl")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status() {
            Ok(status) => status.success(),
            Err(_) => false,
    };

    if installed {
        Ok(None)
    } else {
        Err(KawsError::new("kubectl must be installed".to_string()))
    }
}

fn ensure_terraform() -> KawsResult {
    let installed = match Command::new("terraform")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status() {
            Ok(status) => status.success(),
            Err(_) => false,
    };

    if installed {
        Ok(None)
    } else {
        Err(KawsError::new("terraform must be installed".to_string()))
    }
}
