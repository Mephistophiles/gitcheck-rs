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

fn search_repositories_queue<F>(max_depth: usize, pwd: &PathBuf, f: F) -> Result<()>
where
    F: Fn(&Path),
{
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
                debug!("  Add {} repository", path_entry.path.display());
                f(&path_entry.path);
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

pub(crate) fn search_repositories<F>(max_depth: usize, f: F)
where
    F: Fn(&Path),
{
    let pwd = env::current_dir().unwrap();

    debug!("Beginning scan... building list of git folders");
    debug!("  Scan git repositories from {}", pwd.display());
    let _ = search_repositories_queue(max_depth, &pwd, f);
    debug!("Done");
}
