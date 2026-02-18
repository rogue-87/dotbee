use std::{fs, io, path::Path};

#[derive(Debug, PartialEq)]
pub enum SymlinkStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

pub trait SymlinkManager {
    /// Determines the status of a `destination` path relative to a `source` path
    /// for symlink operations. This is crucial for safely managing dotfiles,
    /// allowing the application to identify existing links, conflicts, or
    /// non-existent paths.
    ///
    /// # Arguments
    /// * `source` - The path to the original file or directory that the symlink
    ///   *should* point to.
    /// * `destination` - The location where the symlink *is* or *would be* created.
    ///
    /// # Returns
    /// A `SymlinkStatus` enum indicating the current state of the `destination` path:
    /// * `AlreadyLinked`: The `destination` is a symlink correctly pointing to `source`.
    /// * `ConflictingFileOrDir`: The `destination` exists but is a regular file or directory.
    /// * `ConflictingSymlink`: The `destination` is a symlink, but it either points
    ///   to a different path than `source` or is broken.
    /// * `NonExistent`: The `destination` path does not exist.
    fn check(&self, source: &Path, destination: &Path) -> SymlinkStatus;

    /// Creates a symbolic link from `source` to `destination`, ensuring all parent
    /// directories of `destination` exist beforehand.
    ///
    /// This function acts like `mkdir -p` followed by `ln -s`.
    ///
    /// # Arguments
    /// * `source` - The path to the original file or directory that the symlink will point to.
    /// * `destination` - The path where the symbolic link will be created.
    ///
    /// # Errors
    /// This function will return an `std::io::Error` if:
    /// * fails to create parent directories.
    /// * fails to create the symbolic link.
    fn create(&self, source: &Path, destination: &Path) -> io::Result<()>;
}

pub struct Symlink;

impl Symlink {
    pub fn new() -> Self {
        Self
    }
}

impl SymlinkManager for Symlink {
    fn check(&self, source: &Path, destination: &Path) -> SymlinkStatus {
        let metadata = match fs::symlink_metadata(destination) {
            Ok(meta) => meta,
            Err(_) => return SymlinkStatus::NonExistent,
        };

        if !metadata.is_symlink() {
            return SymlinkStatus::ConflictingFileOrDir;
        }

        match fs::read_link(destination) {
            Ok(target) if target == source => SymlinkStatus::AlreadyLinked,
            _ => SymlinkStatus::ConflictingSymlink,
        }
    }

    fn create(&self, source: &Path, destination: &Path) -> io::Result<()> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        std::os::unix::fs::symlink(source, destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_symlink_check() -> std::io::Result<()> {
        let dir = tempdir()?;
        let source = dir.path().join("source");
        let destination = dir.path().join("destination");
        let symlink = Symlink::new();

        // Non-existent
        assert_eq!(symlink.check(&source, &destination), SymlinkStatus::NonExistent);

        // File/Dir (not symlink)
        fs::write(&destination, "content")?;
        assert_eq!(symlink.check(&source, &destination), SymlinkStatus::ConflictingFileOrDir);

        // Correct symlink
        fs::remove_file(&destination)?;
        std::os::unix::fs::symlink(&source, &destination)?;
        assert_eq!(symlink.check(&source, &destination), SymlinkStatus::AlreadyLinked);

        // Conflicting symlink
        let other_source = dir.path().join("other_source");
        fs::remove_file(&destination)?;
        std::os::unix::fs::symlink(&other_source, &destination)?;
        assert_eq!(symlink.check(&source, &destination), SymlinkStatus::ConflictingSymlink);

        Ok(())
    }
}
