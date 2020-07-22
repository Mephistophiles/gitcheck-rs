use crate::error::Result;
use log::debug;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct PathEntry {
    path: PathBuf,
    depth: usize,
}

impl PathEntry {
    fn new(path: &Path, depth: usize) -> PathEntry {
        PathEntry {
            path: path.to_path_buf(),
            depth,
        }
    }
}

fn search_repositories_queue(
    max_depth: usize,
    pwd: &PathBuf,
    repos: &mut Vec<PathBuf>,
) -> Result<()> {
    let mut queue = VecDeque::with_capacity(256);

    queue.push_back(PathEntry::new(pwd, 0));

    loop {
        if queue.is_empty() {
            break;
        }

        let mut path_entry = queue.pop_front().unwrap();

        assert!(path_entry.depth <= max_depth);

        for entry in fs::read_dir(&path_entry.path)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().unwrap().is_dir())
        {
            if entry.file_name() == ".git" {
                let copy_pwd = path_entry.path.to_path_buf();
                debug!("  Add {} repository", copy_pwd.display());
                repos.push(copy_pwd);
                continue;
            }

            if path_entry.depth < max_depth {
                path_entry.path.push(entry.file_name());
                queue.push_back(PathEntry::new(&path_entry.path, path_entry.depth + 1));
                path_entry.path.pop();
            }
        }
    }

    Ok(())
}

pub(crate) fn search_repositories(max_depth: usize) -> Vec<PathBuf> {
    let mut repo = Vec::with_capacity(16);
    let pwd = env::current_dir().unwrap();

    debug!("Beginning scan... building list of git folders");
    debug!("  Scan git repositories from {}", pwd.display());
    let _ = search_repositories_queue(max_depth, &pwd, &mut repo);
    debug!("Done");

    repo.sort();

    repo
}
