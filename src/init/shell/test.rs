use super::*;

#[test]
fn test_shell_to_string() {
    assert_eq!(Shell::Fish.to_string(), "fish");
    assert_eq!(Shell::Bash.to_string(), "bash");
    assert_eq!(Shell::Zsh.to_string(), "zsh");
}

#[test]
fn test_load_config() {
    assert!(!Shell::Fish.load_config().is_empty());
    assert!(!Shell::Bash.load_config().is_empty());
    assert!(!Shell::Zsh.load_config().is_empty());
}

#[test]
fn test_service_file_exists() {
    let service_content = include_str!("../../../assets/buddy.service");
    assert!(!service_content.is_empty());
    assert!(service_content.contains("ExecStart=%h/.cargo/bin/buddyd"))
}