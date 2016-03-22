use rusoto::{ChainProvider, ProfileProvider};

pub fn credentials_provider(path: Option<&str>, profile: Option<&str>) -> ChainProvider {
    let mut profile_provider = ProfileProvider::new().expect(
        "Failed to create AWS credentials provider."
    );

    if let Some(path) = path {
        profile_provider.set_file_path(path);
    }

    if let Some(profile) = profile {
        profile_provider.set_profile(profile);
    }

    ChainProvider::with_profile_provider(profile_provider)
}
