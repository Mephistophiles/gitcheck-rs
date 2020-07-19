use crate::error::Result;

use git2::{Repository, Status, StatusOptions};
use std::path::{Path, PathBuf};

pub(crate) struct LocalChanges {
    pub(crate) modified: usize,
}
pub(crate) struct RemoteChanges {
    pub(crate) remote: String,
    pub(crate) ahead: usize,
    pub(crate) behind: usize,
}

// TODO: to &str
pub(crate) struct Changeset {
    path: PathBuf,
    branch: String,
    changes: Vec<Change>,
}

impl Changeset {
    pub(crate) fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
    pub(crate) fn changes(&self) -> &[Change] {
        &self.changes
    }
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
    pub(crate) fn branch(&self) -> &str {
        &self.branch
    }
}

pub(crate) enum Change {
    Local(LocalChanges),
    Remote(RemoteChanges),
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

fn check_local_changes(repo: &Repository, changeset: &mut Vec<Change>) -> Option<()> {
    let mut status_opts = StatusOptions::new();

    if false {
        status_opts.include_untracked(true);
        todo!();
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

fn check_remote_changes(
    repo: &Repository,
    branch: &str,
    remote: &str,
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

pub(crate) fn check_repository(
    repo: &Repository,
    path: PathBuf,
    branch: &str,
) -> Result<Changeset> {
    let mut changeset = vec![];

    check_local_changes(&repo, &mut changeset);

    let remotes = repo.remotes()?;
    for remote in remotes.iter().filter_map(|r| r) {
        check_remote_changes(&repo, branch, remote, &mut changeset);
    }

    Ok(Changeset {
        path,
        branch: branch.to_string(),
        changes: changeset,
    })
}
