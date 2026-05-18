use std::{fs, path::Path};

use clap::builder::OsStr;
use colored::Colorize;

use crate::{constants::ProjectInfo, scanner::get_removable_space_bytes, utils::bytes_to_gb};

pub const ALLOWED_TIME_UNITS: [char; 3] = ['d', 'm', 'y'];

fn remove_dir(pb: &Path, is_apply: bool) {
    if is_apply {
        if let Err(e) = fs::remove_dir_all(pb) {
            eprintln!("Warning: failed to remove {}: {e}", pb.display());
        }
        println!("removed directory: {}", pb.display());
    } else {
        println!("[dry-run] would remove: {}", pb.display());
    }
}

fn remove_dirs(pi: &ProjectInfo, is_apply: bool, is_hard: bool) {
    if is_hard {
        remove_dir(&pi.path, is_apply);
    }

    pi.template
        .dirs_to_clear
        .iter()
        .filter(|rd| pi.path.join(rd).is_dir())
        .map(|rd| pi.path.join(rd))
        .for_each(|pb| remove_dir(&pb, is_apply));
}

pub fn remove_all_removable_dirs(pi_vec: &[ProjectInfo], is_apply: bool, hard: bool) {
    if !is_apply {
        println!();
        println!("{}", "Directories to be removed:".red());
    }

    let mut total_cleared = 0;

    pi_vec.iter().for_each(|pi| {
        let fallback = &OsStr::from("unknown");
        let project_name = pi.path.file_name().unwrap_or(fallback).to_string_lossy();

        if is_apply {
            println!("Cleaning project: {}", project_name.white());
        } else {
            println!("[dry-run] Cleaning project: {}", project_name.white());
        }

        total_cleared += get_removable_space_bytes(pi, hard);
        remove_dirs(pi, is_apply, hard);
    });

    if !is_apply {
        println!("{}", "—".repeat(30).red());
    } else {
        println!(
            "\n▶ Total space cleared: ~ {} GiB",
            format!("{:.2}", bytes_to_gb(total_cleared)).red()
        )
    }
}
