use std::path::PathBuf;

/// Expands a tilde (`~`) at the start of a path to the user's home directory.
///
/// # Arguments
/// * `path_str` - A string slice representing the path to expand.
///
/// # Returns
/// A `PathBuf` with the tilde expanded, or the original path if no tilde was present.
pub fn expand_tilde(path_str: &str) -> PathBuf {
    match dirs::home_dir() {
        // simply return home
        Some(home) if path_str == "~" => home,
        // slice it after ~/ (same as strip_prefix()) then join it with dirs::home_dir()
        Some(home) if path_str.starts_with("~/") => home.join(&path_str[2..]),
        // do nothing
        _ => PathBuf::from(path_str),
    }
}

/// Retrieves the system's hostname.
///
/// # Panics
/// This function will panic if it fails to get the hostname from the system or
/// if the hostname cannot be parsed into a valid string.
pub fn get_hostname() -> String {
    use nix::unistd::gethostname;
    let hostname = gethostname().expect("Couldn't get hostname");
    hostname.into_string().expect("Failed to parse hostname")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        if let Some(home) = dirs::home_dir() {
            assert_eq!(expand_tilde("~"), home);
            assert_eq!(expand_tilde("~/test"), home.join("test"));
        }
        assert_eq!(expand_tilde("/etc/hosts"), PathBuf::from("/etc/hosts"));
    }
}
