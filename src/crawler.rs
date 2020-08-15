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
use crate::error::Result;
use jwalk::{ClientState, DirEntry, WalkDir};
use log::debug;
use std::env;
use std::path::Path;

fn filter_path<C: ClientState>(dir_entry: &jwalk::Result<DirEntry<C>>) -> bool {
    let dir_entry = match dir_entry {
        Ok(e) => e,
        Err(_) => return false,
    };

    if !dir_entry.file_type.is_dir() {
        return false;
    }

    let file_name = match dir_entry.file_name().to_str() {
        Some(f) => f,
        None => return false,
    };

    if file_name.starts_with('.') && file_name != ".git" {
        return false;
    }

    true
}

fn search_repositories_parallel<F>(max_depth: usize, pwd: &Path, f: F) -> Result<()>
where
    F: Fn(&Path),
{
    let walker = WalkDir::new(pwd)
        .skip_hidden(false)
        .max_depth(max_depth)
        .process_read_dir(move |_read_dir_state, children| {
            children.retain(filter_path);

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
