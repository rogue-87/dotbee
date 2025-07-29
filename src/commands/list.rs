use colored::Colorize;
use std::path::Path;
use walkdir::WalkDir;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Available {} and {}", "hosts".green(), "configs".yellow());

    let root = Path::new("hosts");

    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        let relative = path.strip_prefix(root)?;
        let depth = relative.components().count();
        let indent = "  ".repeat(depth);

        let name = relative.file_name().unwrap().to_string_lossy();

        match depth {
            1 => println!("{}{}", indent, name.green().bold()), // hosts
            _ => println!("{}{}", indent, name.yellow()),       // configs
        }
    }

    Ok(())
}
