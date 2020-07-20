use log::debug;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn search_repositories(max_depth: usize) -> Vec<PathBuf> {
    let mut repo = Vec::with_capacity(16);
    let pwd = env::current_dir().unwrap();

    debug!("Beginning scan... building list of git folders");
    debug!("  Scan git repositories from {}", pwd.display());

    for entry in WalkDir::new(&pwd)
        .max_depth(max_depth)
        .same_file_system(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        if entry.file_name() == ".git" {
            let mut path = entry.into_path();
            path.pop();

            debug!("  Add {} repository", path.display());
            repo.push(path);
        }
    }
    debug!("Done");

    repo.sort();

    repo
}
