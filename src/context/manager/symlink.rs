use std::{error::Error, fs, path::Path};

#[derive(Debug, PartialEq)]
pub enum SymlinkStatus {
    AlreadyLinked,
    ConflictingFileOrDir,
    ConflictingSymlink,
    NonExistent,
}

pub struct SymlinkManager;

impl SymlinkManager {
    pub fn new() -> Self {
        Self
    }

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
    pub fn check(&self, source: &Path, destination: &Path) -> SymlinkStatus {
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
    /// This function will return an error if:
    /// * fails to create parent directories.
    /// * fails to create the symbolic link.
    pub fn create(&self, source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        std::os::unix::fs::symlink(source, destination)?;
        Ok(())
    }

    /// Force creates a symlink by removing any existing file or directory at the destination.
    /// This is a convenience method for handling conflicts automatically.
    ///
    /// # Arguments
    /// * `source` - The path to the original file or directory that the symlink will point to.
    /// * `destination` - The path where the symbolic link will be created.
    ///
    /// # Errors
    /// Returns an error if removal of existing files or symlink creation fails.
    pub fn _force_create(&self, source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        // Remove existing file/directory/symlink if it exists
        self._remove_existing(destination)?;
        self.create(source, destination)
    }

    /// Safely removes an existing file, directory, or symlink at the given path.
    /// Does nothing if the path doesn't exist.
    ///
    /// # Arguments
    /// * `path` - The path to remove.
    ///
    /// # Errors
    /// Returns an error if removal fails (but not if path doesn't exist).
    pub fn _remove_existing(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            return Ok(());
        }

        let metadata = fs::symlink_metadata(path)?;

        if metadata.is_symlink() {
            fs::remove_file(path)?;
        } else if metadata.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }

        Ok(())
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
        let symlink_manager = SymlinkManager::new();

        // Non-existent
        assert_eq!(symlink_manager.check(&source, &destination), SymlinkStatus::NonExistent);

        // File/Dir (not symlink)
        fs::write(&destination, "content")?;
        assert_eq!(symlink_manager.check(&source, &destination), SymlinkStatus::ConflictingFileOrDir);

        // Correct symlink
        fs::remove_file(&destination)?;
        std::os::unix::fs::symlink(&source, &destination)?;
        assert_eq!(symlink_manager.check(&source, &destination), SymlinkStatus::AlreadyLinked);

        // Conflicting symlink
        let other_source = dir.path().join("other_source");
        fs::remove_file(&destination)?;
        std::os::unix::fs::symlink(&other_source, &destination)?;
        assert_eq!(symlink_manager.check(&source, &destination), SymlinkStatus::ConflictingSymlink);

        Ok(())
    }

    #[test]
    fn test_symlink_create() -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;
        let source = dir.path().join("source");
        let destination = dir.path().join("nested").join("destination");
        let symlink_manager = SymlinkManager::new();

        // Create source file
        fs::write(&source, "test content")?;

        // Create symlink (should create parent directory)
        symlink_manager.create(&source, &destination)?;

        // Verify symlink was created correctly
        assert!(destination.is_symlink());
        assert_eq!(fs::read_link(&destination)?, source);

        Ok(())
    }

    #[test]
    fn test_force_create() -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;
        let source = dir.path().join("source");
        let destination = dir.path().join("destination");
        let symlink_manager = SymlinkManager::new();

        // Create source file
        fs::write(&source, "test content")?;

        // Create conflicting file at destination
        fs::write(&destination, "conflicting content")?;

        // Force create should remove the conflicting file and create symlink
        symlink_manager._force_create(&source, &destination)?;

        // Verify symlink was created correctly
        assert!(destination.is_symlink());
        assert_eq!(fs::read_link(&destination)?, source);

        Ok(())
    }

    #[test]
    fn test_remove_existing() -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;
        let symlink_manager = SymlinkManager::new();

        // Test removing non-existent file (should not error)
        let non_existent = dir.path().join("non_existent");
        symlink_manager._remove_existing(&non_existent)?;

        // Test removing regular file
        let file_path = dir.path().join("test_file");
        fs::write(&file_path, "content")?;
        assert!(file_path.exists());
        symlink_manager._remove_existing(&file_path)?;
        assert!(!file_path.exists());

        // Test removing directory
        let dir_path = dir.path().join("test_dir");
        fs::create_dir(&dir_path)?;
        fs::write(dir_path.join("file_in_dir"), "content")?;
        assert!(dir_path.exists());
        symlink_manager._remove_existing(&dir_path)?;
        assert!(!dir_path.exists());

        // Test removing symlink
        let source = dir.path().join("source");
        let symlink_path = dir.path().join("test_symlink");
        fs::write(&source, "content")?;
        std::os::unix::fs::symlink(&source, &symlink_path)?;
        assert!(symlink_path.is_symlink());
        symlink_manager._remove_existing(&symlink_path)?;
        assert!(!symlink_path.exists());

        Ok(())
    }
}
