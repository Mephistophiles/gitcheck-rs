use log::debug;
use std::collections::BTreeSet;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn search_repositories(max_depth: usize) -> BTreeSet<PathBuf> {
    let mut repo = BTreeSet::new();
    let pwd = env::current_dir().unwrap();

    debug!("Beginning scan... building list of git folders");
    for entry in WalkDir::new(&pwd)
        .max_depth(max_depth)
        .same_file_system(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        debug!("  Scan git repositories from {}", entry.path().display());

        if entry.file_name() == ".git" {
            let path = entry.path().strip_prefix(&pwd).unwrap();
            let mut path = path.to_path_buf();
            path.pop();

            debug!("  Add {} repository", path.display());
            repo.insert(path);
        }
    }
    debug!("Done");

    repo
}
