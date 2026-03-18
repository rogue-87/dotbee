use dotbee::context::manager::Manager;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

/// Setup a test environment with a temporary HOME to avoid touching the user's real state.
fn setup_test_env() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let state_dir = dir.path().join(".local/state/dotbee");
    fs::create_dir_all(&state_dir).unwrap();

    unsafe {
        std::env::set_var("HOME", dir.path());
        std::env::set_var("XDG_STATE_HOME", dir.path().join(".local/state"));
    }
    dir
}

fn write_config(dir: &std::path::Path, content: &str) -> PathBuf {
    let path = dir.join("dotbee.toml");
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_manager_initialization() {
    let _env = setup_test_env();
    let dir = tempdir().unwrap();
    let toml = r#"
[profiles.desktop.links]
"~/.bashrc" = "bashrc"
"#;
    let config_path = write_config(dir.path(), toml);

    let manager = Manager::new(Some(config_path.to_str().unwrap().to_string())).expect("Failed to create manager");

    assert!(manager.config.has_profiles());
    assert!(manager.config.list_profiles().contains(&"desktop"));
    assert_eq!(manager.state.get_active_profile(), None);
}

#[test]
fn test_manager_state_persistence() {
    let _env = setup_test_env();
    let dir = tempdir().unwrap();
    let config_path = write_config(dir.path(), "");

    {
        let mut manager = Manager::new(Some(config_path.to_str().unwrap().to_string())).unwrap();
        manager.state.set_active_profile("laptop".to_string()).unwrap();
    }

    // Reload and check if state persisted
    let manager = Manager::new(Some(config_path.to_str().unwrap().to_string())).unwrap();
    assert_eq!(manager.state.get_active_profile(), Some("laptop"));
}

#[test]
fn test_manager_config_loading() {
    let _env = setup_test_env();
    let dir = tempdir().unwrap();
    let toml = r#"
[global.links]
"~/.gitconfig" = "gitconfig"
"#;
    let config_path = write_config(dir.path(), toml);

    let manager = Manager::new(Some(config_path.to_str().unwrap().to_string())).unwrap();
    let global_links = manager.config.get_global_links().expect("Missing global links");
    assert_eq!(global_links.get("~/.gitconfig").unwrap(), "gitconfig");
}

#[test]
fn test_manager_symlink_check() {
    let _env = setup_test_env();
    let dir = tempdir().unwrap();
    let manager = Manager::new(None).unwrap();

    let source = dir.path().join("source");
    let target = dir.path().join("target");
    fs::write(&source, "content").unwrap();

    // Initially non-existent
    assert_eq!(
        manager.symlink.check(&source, &target),
        dotbee::context::manager::symlink::SymlinkStatus::NonExistent
    );

    // Create link
    manager.symlink.create(&source, &target).unwrap();
    assert_eq!(
        manager.symlink.check(&source, &target),
        dotbee::context::manager::symlink::SymlinkStatus::AlreadyLinked
    );
}
