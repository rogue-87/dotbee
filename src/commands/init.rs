use dirs::home_dir;
use std::fs;

pub fn run(name: String) {
    let home = match home_dir() {
        Some(dir) => dir,
        None => {
            eprintln!("❌ Could not determine home directory.");
            return;
        }
    };

    let dotfiles_dir = home.join(&name);
    let hosts_dir = dotfiles_dir.join("hosts");
    let global_dir = dotfiles_dir.join("global");

    if dotfiles_dir.exists() {
        eprintln!(
            "❌ Dotfiles folder '{}' already exists.",
            dotfiles_dir.display()
        );
        return;
    }

    if let Err(err) = fs::create_dir_all(&hosts_dir) {
        eprintln!("❌ Failed to create 'hosts' folder: {err}");
        return;
    }

    if let Err(err) = fs::create_dir_all(&global_dir) {
        eprintln!("❌ Failed to create 'global' folder: {err}");
        return;
    }

    println!("✅ Initialized dotfiles at '{}'", dotfiles_dir.display());
}
