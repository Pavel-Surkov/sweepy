use std::fs;

use clap::builder::OsStr;
use colored::Colorize;

use crate::constants::ProjectInfo;

pub const ALLOWED_TIME_UNITS: [char; 3] = ['d', 'm', 'y'];

fn remove_removable_dirs(pi: &ProjectInfo, is_apply: bool) {
    pi.template
        .dirs_to_clear
        .iter()
        .filter(|rd| pi.path.join(rd).is_dir())
        .map(|rd| pi.path.join(rd))
        .for_each(|pb| {
            if is_apply {
                if let Err(e) = fs::remove_dir_all(&pb) {
                    eprintln!("Warning: failed to remove {}: {e}", pb.display());
                }
                println!("removed directory: {}", pb.display());
            } else {
                println!("[dry-run] would remove: {}", pb.display());
            }
        });
}

pub fn remove_all_removable_dirs(pi_vec: &[ProjectInfo], is_apply: bool) {
    if !is_apply {
        println!();
        println!("{}", "Directories to be removed:".red());
    }

    pi_vec.iter().for_each(|pi| {
        let fallback = &OsStr::from("unknown");
        let project_name = pi.path.file_name().unwrap_or(fallback).to_string_lossy();

        if is_apply {
            println!("Cleaning project: {}", project_name.white());
        } else {
            println!("[dry-run] Cleaning project: {}", project_name.white());
        }

        remove_removable_dirs(pi, is_apply);
    });

    if !is_apply {
        println!("{}", "—".repeat(30).red());
    }
}
