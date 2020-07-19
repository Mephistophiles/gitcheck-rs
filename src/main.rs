use ansi_term::Colour::White;

use crate::git::{Change, Changeset};

use crossbeam::crossbeam_channel;
use log::debug;
use rayon::prelude::*;
use std::path::Path;
use std::{env, thread};

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

fn print_changes(changeset: Changeset) {
    if !changeset.has_changes() {
        report_unchanged_repo(changeset.path(), changeset.branch());
        return;
    }

    report_modified_repo(changeset.path(), changeset.branch());

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

fn main() {
    let matches = cmdline::parse_args();

    if let Some(dir) = matches.value_of("dir") {
        env::set_current_dir(dir).unwrap();
    }

    if matches.is_present("debug") {
        flexi_logger::Logger::with_str("debug").start().unwrap();
    } else {
        flexi_logger::Logger::with_env().start().unwrap();
    }
    debug!("Global Vars: {:?}", env::args());

    let max_depth: usize = match matches.value_of("maxdepth") {
        Some(i) => i.parse().unwrap(),
        None => usize::MAX,
    };

    debug!("Processing repositories... please wait.");
    let repos = crawler::search_repositories(max_depth);
    let ignore_branch_regex;
    let check_all = matches.is_present("all-branch");

    if let Some(li) = matches.value_of("ignore-branch") {
        ignore_branch_regex = Some(regex::Regex::new(li).unwrap());
    } else {
        ignore_branch_regex = None;
    }

    let (send, recv) = crossbeam_channel::unbounded();

    thread::spawn(move || {
        repos.par_iter().for_each(|path| {
            let repo = git2::Repository::open(&path).unwrap();
            let branches;

            if check_all {
                branches = git::get_all_branches(&repo);
            } else {
                branches = git::get_default_branch(&repo);
            }

            for branch in branches {
                if let Some(ref ignore_regex) = ignore_branch_regex {
                    if ignore_regex.is_match(&branch) {
                        continue;
                    }
                }

                if let Ok(changeset) = git::check_repository(&repo, path.clone(), &branch) {
                    send.send(changeset).unwrap();
                }
            }
        });

        drop(send);
    });

    for change in recv.iter() {
        print_changes(change);
    }
}
