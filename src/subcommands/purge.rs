use crate::utils::common::expand_tilde;
use crate::{context::Context, utils::message};
use colored::Colorize;
use std::{fs, io, path::PathBuf};

/// The types of actions our purge command can take.
pub enum Action {
    Delete { target_display: String, path: PathBuf },
    NotifyMissing { target_display: String },
    NotifyNotASymlink { target_display: String, _path: PathBuf },
}

pub fn run(context: &mut Context) -> anyhow::Result<(), anyhow::Error> {
    // 1. GENERATE THE PLAN
    // This always runs, fixing the previous dry-run bug.
    let plan = generate_plan(context);

    if plan.is_empty() {
        message::info("No managed links found to purge.");

        if !context.dry_run {
            context.manager.state.clear()?;
            message::success("State cleared.");
        }
        return Ok(());
    }

    // 2. DISPATCH
    match context.dry_run {
        true => execute_dry(&plan),
        false => execute(plan, context)?,
    }

    Ok(())
}

fn generate_plan(context: &Context) -> Vec<Action> {
    let mut plan: Vec<Action> = vec![];

    for link in context.manager.state.get_links() {
        let target_path = expand_tilde(&link.target);

        // Check if the path exists or is a broken symlink
        if !target_path.exists() && !target_path.is_symlink() {
            plan.push(Action::NotifyMissing {
                target_display: link.target.clone(),
            });
            continue;
        }

        // Safety check: Is it actually a symlink?
        if !target_path.is_symlink() {
            plan.push(Action::NotifyNotASymlink {
                target_display: link.target.clone(),
                _path: target_path,
            });
            continue;
        }

        plan.push(Action::Delete {
            target_display: link.target.clone(),
            path: target_path,
        });
    }

    plan
}

fn execute(plan: Vec<Action>, context: &mut Context) -> anyhow::Result<(), anyhow::Error> {
    println!("{}", "Executing Purge...".bold().red());

    for action in plan {
        match action {
            Action::Delete { path, target_display } => match fs::remove_file(&path) {
                Ok(_) => {
                    message::delete(&format!("Removed {}", target_display));
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        message::warning(&format!("Target '{}' disappeared during execution.", target_display));
                    } else {
                        message::error(&format!("Failed to remove {}: {}", target_display, e));
                    }
                }
            },
            Action::NotifyMissing { target_display } => {
                message::warning(&format!("Cleaning up stale state for missing link: {}", target_display));
            }
            Action::NotifyNotASymlink { target_display, .. } => {
                message::error(&format!("Aborting removal of {}: path is a real file/directory.", target_display));
            }
        }
    }

    context.manager.state.clear()?;
    message::success("Purge complete.");
    Ok(())
}

fn execute_dry(plan: &[Action]) {
    println!("{}", "Purge Plan (Dry Run):".bold().yellow());

    for action in plan {
        match action {
            Action::Delete { target_display, .. } => {
                message::delete(&format!("Would remove {}", target_display));
            }
            Action::NotifyMissing { target_display, .. } => {
                message::warning(&format!("{} is already missing from disk.", target_display));
            }
            Action::NotifyNotASymlink { target_display, .. } => {
                message::error(&format!("SKIPPING {}: not a symlink.", target_display));
            }
        }
    }
}
