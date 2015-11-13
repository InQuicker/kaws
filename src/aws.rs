use std::env::home_dir;
use std::path::PathBuf;

use ini::Ini;

use error::Error;

pub fn get_credentials<'a>(path: Option<&'a str>, profile: Option<&'a str>)
    -> Result<(String, String), Error>
{
    let final_path = match path {
        Some(path) => PathBuf::from(path),
        None => {
            let mut pathbuf = try!(
                home_dir().ok_or(Error::new(format!("Home directory could not be determined")))
            );
            pathbuf.push(".aws");
            pathbuf.push("credentials");

            pathbuf
        },
    };

    let conf = try!(
        Ini::load_from_file(try!(final_path.to_str().ok_or(Error::new(format!("Invalid unicode")))))
    );

    let section = try!(
        conf.section(Some(profile.unwrap_or("default"))).ok_or(
            Error::new(format!("default profile not found"))
        )
    );
    let aws_access_key_id = try!(
        section.get("aws_access_key_id").ok_or(Error::new(format!("Could not read access key ID")))
    );
    let aws_secret_access_key = try!(
        section.get("aws_access_key_id").ok_or(
            Error::new(format!("Could not read secret access key"))
        )
    );

    Ok((aws_access_key_id.clone(), aws_secret_access_key.clone()))
}
