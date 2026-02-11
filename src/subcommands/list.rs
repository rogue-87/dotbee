use colored::Colorize;
use context::Context;
use utils::{find_active_profile, is_profile_active};
use indexmap::IndexMap;
use std::error::Error;

pub fn run(context: &Context) -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;

    if let Some(global) = &context.config.global {
        println!("{}", "global".yellow().bold());
        print_links(&global.links);
    }

    let active_profile_name = if let Some(profiles) = &context.config.profiles {
        find_active_profile(profiles, context.state.active_profile.as_ref(), &cwd)
    } else {
        None
    };

    if let Some(profiles) = &context.config.profiles {
        for (name, profile) in profiles {
            let is_resolved_active = active_profile_name == Some(&name);
            let is_active_in_state = context.state.active_profile.as_ref() == Some(&name);
            let is_physically_active = is_profile_active(&profile, &cwd);

            let title = if is_resolved_active {
                if is_active_in_state {
                    if is_physically_active {
                        format!("{} (active)", name).green().bold()
                    } else {
                        format!("{} (active - broken)", name).yellow().bold()
                    }
                } else {
                    format!("{} (active - inferred)", name).cyan().bold()
                }
            } else {
                name.bold()
            };

            println!("{}", title);
            print_links(&profile.links);
        }
    } else if context.config.global.is_none() {
        println!("No profiles or global links defined in config.");
    }

    Ok(())
}

fn print_links(links: &IndexMap<String, String>) {
    let mut links_vec: Vec<_> = links.iter().collect();
    links_vec.sort_by_key(|(k, _)| k.as_str());

    for (i, (target, source)) in links_vec.iter().enumerate() {
        let is_last = i == links_vec.len() - 1;
        let branch = if is_last { "└──" } else { "├──" };
        println!("{} {} -> {}", branch, target, source);
    }
}
