use std::fs;

pub fn run(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;

    let dotfiles_dir = cwd.join(&name);
    let hosts_dir = dotfiles_dir.join("hosts");
    let global_dir = dotfiles_dir.join("global");

    if dotfiles_dir.exists() {
        return Err(format!("Folder '{}' already exists!", dotfiles_dir.display()).into());
    }

    fs::create_dir_all(&hosts_dir)?;
    fs::create_dir(&global_dir)?;

    Ok(())
}
