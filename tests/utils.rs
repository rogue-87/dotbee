use dotbee::utils::expand_tilde;
use std::path::PathBuf;

#[test]
fn test_expand_tilde() {
    if let Some(home) = dirs::home_dir() {
        assert_eq!(expand_tilde("~"), home);
        assert_eq!(expand_tilde("~/test"), home.join("test"));
    }
    assert_eq!(expand_tilde("/etc/hosts"), PathBuf::from("/etc/hosts"));
}
