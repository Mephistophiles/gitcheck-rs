use crate::error::Result;
use jwalk::WalkDir;
use log::debug;
use std::env;
use std::path::{Path, PathBuf};

fn search_repositories_parallel<F>(max_depth: usize, pwd: &PathBuf, f: F) -> Result<()>
where
    F: Fn(&Path),
{
    let walker = WalkDir::new(pwd)
        .skip_hidden(false)
        .max_depth(max_depth)
        .process_read_dir(move |_read_dir_state, children| {
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        if !dir_entry.file_type.is_dir() {
                            return false;
                        }

                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            if file_name.starts_with('.') && file_name != ".git" {
                                return false;
                            }

                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            });

            children.iter_mut().for_each(|dir_entry_result| {
                if let Ok(dir_entry) = dir_entry_result {
                    // reached maximum depth
                    if dir_entry.depth == max_depth {
                        dir_entry.read_children_path = None;
                    }

                    // in .git folder nothing to show
                    if dir_entry.file_name == ".git" {
                        dir_entry.read_children_path = None;
                    }
                }
            });
        });

    for entry in walker {
        let entry = entry?;

        if entry.file_name() == ".git" {
            let parent = entry.parent_path();
            debug!("  Add {} repository", parent.display());
            f(parent);
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
    let _ = search_repositories_parallel(max_depth, &pwd, f);
    debug!("Done");
}
