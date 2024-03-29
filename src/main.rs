/*
 *  Copyright (C) 2020 Maxim Zhukov <mussitantesmortem@gmail.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use ansi_term::Colour::White;

use crate::cmdline::Options;
use crate::git::{Change, Changeset};

use crate::error::Result;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::sync::WaitGroup;
use log::debug;
use std::path::{Path, PathBuf};
use std::thread;
use xshell::{cmd, Shell};

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

fn print_changes(cwd: &Path, changeset: Changeset) {
    let mut repo_path = changeset
        .path()
        .strip_prefix(&cwd)
        .unwrap_or_else(|_| changeset.path());
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

fn update_remote(path: &Path) -> Result<()> {
    debug!("Updating {} remotes...", path.display());
    let sh = Shell::new()?;

    cmd!(sh, "git -C {path} remote update").run().unwrap();

    Ok(())
}

fn process_repo(cwd: &Path, path: &Path, args: &Options) {
    let repo = git2::Repository::open(path).unwrap();

    if args.remote {
        match update_remote(path) {
            Ok(_) => (),
            Err(e) => eprintln!("Update error: {}", e),
        }
    }

    let branches = if args.all_branch {
        git::get_all_branches(&repo)
    } else {
        git::get_default_branch(&repo)
    };

    for branch in branches {
        if let Some(ref ignore_regex) = args.ignore_branch_regex {
            if ignore_regex.is_match(&branch) {
                continue;
            }
        }

        if let Ok(changeset) = git::check_repository(&repo, path, &branch, args) {
            if !args.quiet || changeset.has_changes() {
                print_changes(cwd, changeset);
            }
        }
    }
}

fn main() {
    let args = cmdline::parse_args();

    flexi_logger::Logger::try_with_env_or_str(if args.debug { "debug" } else { "info" })
        .unwrap()
        .start()
        .unwrap();

    // Create a channel of unbounded capacity.
    let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = crossbeam_channel::unbounded();

    // Create a new wait group.
    let wg = WaitGroup::new();

    debug!("Processing repositories... please wait.");

    for _ in 0..args.jobs {
        // get current working directory
        let cwd = std::env::current_dir().unwrap();
        let rx = rx.clone();
        let wg = wg.clone();
        let args = args.clone();

        thread::spawn(move || {
            for path in rx.iter() {
                process_repo(&cwd, &path, &args);
            }

            drop(wg);
        });
    }

    for dir in args.working_directories {
        crawler::search_repositories(args.max_depth, &dir, |path| {
            tx.send(path).unwrap();
        });
    }
    drop(tx);

    wg.wait();
}
