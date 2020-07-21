use crate::error::Result;
use log::debug;
use std::env;
use std::fs;
use std::path::PathBuf;

fn search_repositories_iter(
    current_depth: usize,
    max_depth: usize,
    pwd: &mut PathBuf,
    repos: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(&pwd)?.filter_map(|e| e.ok()) {
        if !entry.file_type()?.is_dir() {
            continue;
        }

        if entry.file_name() == ".git" {
            let copy_pwd = pwd.to_path_buf();
            debug!("  Add {} repository", copy_pwd.display());
            repos.push(copy_pwd);
        }

        if current_depth < max_depth {
            pwd.push(entry.file_name());
            let _ = search_repositories_iter(current_depth + 1, max_depth, pwd, repos);
            pwd.pop();
        }
    }

    Ok(())
}

pub(crate) fn search_repositories(max_depth: usize) -> Vec<PathBuf> {
    let mut repo = Vec::with_capacity(16);
    let mut pwd = env::current_dir().unwrap();

    debug!("Beginning scan... building list of git folders");
    debug!("  Scan git repositories from {}", pwd.display());
    let _ = search_repositories_iter(0, max_depth, &mut pwd, &mut repo);
    debug!("Done");

    repo.sort();

    repo
}
