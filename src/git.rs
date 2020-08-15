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

use git2::{Repository, Status, StatusOptions};
use std::path::Path;

pub(crate) struct LocalChanges {
    pub(crate) modified: usize,
}
pub(crate) struct RemoteChanges {
    pub(crate) remote: String,
    pub(crate) ahead: usize,
    pub(crate) behind: usize,
}

// TODO: to &str
pub(crate) struct Changeset<'a> {
    path: &'a Path,
    branch: &'a str,
    changes: Vec<Change>,
}

pub(crate) enum Change {
    Local(LocalChanges),
    Remote(RemoteChanges),
}

impl<'a> Changeset<'a> {
    pub(crate) fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
    pub(crate) fn changes(&self) -> &[Change] {
        &self.changes
    }
    pub(crate) fn path(&self) -> &Path {
        self.path
    }
    pub(crate) fn branch(&self) -> &str {
        &self.branch
    }
}

pub(crate) fn get_all_branches(repo: &Repository) -> Vec<String> {
    let branches = repo.branches(None).unwrap();
    let mut branch_out = Vec::new();

    for branch in branches.filter_map(|e| {
        let branch = match e {
            Ok((b, _)) => b,
            _ => return None,
        };

        match branch.name() {
            Ok(Some(b)) => Some(b.to_string()),
            _ => None,
        }
    }) {
        branch_out.push(branch);
    }

    branch_out
}

pub(crate) fn get_default_branch(repo: &Repository) -> Vec<String> {
    let mut branch_out = Vec::with_capacity(1);

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return branch_out,
    };
    let branch = head.shorthand().unwrap();
    branch_out.push(branch.to_string());

    branch_out
}

fn check_local_changes(
    repo: &Repository,
    changeset: &mut Vec<Change>,
    args: &crate::Options,
) -> Option<()> {
    let mut status_opts = StatusOptions::new();

    if args.untracked {
        status_opts.include_untracked(true);
    }

    let statuses = repo.statuses(Some(&mut status_opts)).ok()?;

    let changed = statuses
        .iter()
        .map(|e| e.status())
        .filter(|s| {
            s.intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE
                    | Status::WT_MODIFIED
                    | Status::WT_NEW
                    | Status::WT_RENAMED
                    | Status::WT_TYPECHANGE,
            )
        })
        .count();

    if changed > 0 {
        changeset.push(Change::Local(LocalChanges { modified: changed }));
    }

    Some(())
}

fn check_remote_changes<'a>(
    repo: &Repository,
    branch: &'a str,
    remote: &'a str,
    changeset: &mut Vec<Change>,
) -> Option<()> {
    let local = repo.revparse_single("HEAD").ok()?;
    let upstream = repo
        .revparse_single(&format!("{}/{}", remote, branch))
        .ok()?;

    let (ahead, behind) = repo.graph_ahead_behind(local.id(), upstream.id()).ok()?;

    if ahead > 0 || behind > 0 {
        changeset.push(Change::Remote(RemoteChanges {
            remote: remote.to_string(),
            ahead,
            behind,
        }));
    }

    Some(())
}

pub(crate) fn check_repository<'a, 'b>(
    repo: &'b Repository,
    path: &'a Path,
    branch: &'a str,
    args: &crate::Options,
) -> Result<Changeset<'a>> {
    let mut changeset = vec![];

    check_local_changes(&repo, &mut changeset, args);

    let remotes = repo.remotes()?;
    for remote in remotes.iter().filter_map(|r| r) {
        check_remote_changes(&repo, &branch, remote, &mut changeset);
    }

    Ok(Changeset {
        path,
        branch,
        changes: changeset,
    })
}
