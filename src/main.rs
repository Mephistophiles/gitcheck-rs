use ansi_term::Colour::White;

use crate::cmdline::Options;
use crate::git::{Change, Changeset};

use log::debug;
use rayon::prelude::*;
use std::env;
use std::path::Path;

mod cmdline;
mod colors;
mod crawler;
mod error;
mod git;

fn report_modified_repo(path: &Path, branch: &str) {
    print!(
        "{}/{}",
        colors::DEEP_PINK_1A.bold().paint(path.to_string_lossy()),
        White.paint(branch)
    );
}

fn report_unchanged_repo(path: &Path, branch: &str) {
    println!(
        "{}/{}",
        colors::CHARTREUSE_1.paint(path.to_string_lossy()),
        colors::WHITE.paint(branch)
    );
}

fn print_stat(stat: usize, origin: &str, name: &str) {
    print!(
        " {}{}{}{}{}",
        colors::LIGHT_GOLDENROD_2B.paint(origin),
        colors::WHITE.paint("["),
        colors::DEEP_SKY_BLUE_3B.paint(name),
        colors::WHITE.paint(format!("{}", stat)),
        colors::WHITE.paint("]")
    );
}

fn print_changes(pwd: &Path, changeset: Changeset) {
    let mut repo_path = changeset.path().strip_prefix(&pwd).unwrap();
    if repo_path == Path::new("") {
        repo_path = Path::new(changeset.path().file_name().unwrap());
    }

    if !changeset.has_changes() {
        report_unchanged_repo(repo_path, changeset.branch());
        return;
    }

    report_modified_repo(repo_path, changeset.branch());

    for change in changeset.changes() {
        match change {
            Change::Local(changes) => {
                print_stat(changes.modified, "Local", "To Commit:");
            }
            Change::Remote(changes) => {
                if changes.ahead > 0 {
                    print_stat(changes.ahead, &changes.remote, "To Push:");
                }
                if changes.behind > 0 {
                    print_stat(changes.behind, &changes.remote, "To Pull:");
                }
            }
        }
    }
    println!();
}

fn process_repo(path: &Path, args: &Options) {
    let repo = git2::Repository::open(path).unwrap();
    let branches;

    if args.all_branch {
        branches = git::get_all_branches(&repo);
    } else {
        branches = git::get_default_branch(&repo);
    }

    for branch in branches {
        if let Some(ref ignore_regex) = args.ignore_branch_regex {
            if ignore_regex.is_match(&branch) {
                continue;
            }
        }

        if let Ok(changeset) = git::check_repository(&repo, path, &branch) {
            if !args.quiet || changeset.has_changes() {
                print_changes(&args.working_directory, changeset);
            }
        }
    }
}

fn main() {
    let args = cmdline::parse_args();

    env::set_current_dir(&args.working_directory).unwrap();

    if args.debug {
        flexi_logger::Logger::with_str("debug").start().unwrap();
    } else {
        flexi_logger::Logger::with_env().start().unwrap();
    }

    let repos = crawler::search_repositories(args.max_depth);

    if args.jobs > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.jobs)
            .build_global()
            .unwrap();
    }

    debug!("Processing repositories... please wait.");

    repos.par_iter().for_each(|path| {
        process_repo(&path, &args);
    });
}
