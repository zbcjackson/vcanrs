use std::path::PathBuf;
use git2::{DiffFindOptions, Patch, Repository, Time};
use time::Tm;

#[cfg(test)]
use mockall::{automock};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Delta {
    pub(crate) old_file: String,
    pub(crate) new_file: String,
    pub(crate) status: DeltaStatus,
    pub(crate) lines: i32,
}

#[derive(Debug, Clone)]
pub enum DeltaStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
    Other
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Commit {
    pub(crate) id: String,
    pub(crate) message: String,
    pub(crate) time: Tm,
    pub(crate) author: String,
    pub(crate) deltas: Vec<Delta>
}

#[cfg_attr(test, automock)]
pub trait Repo {
    fn commits(&self) -> Vec<Commit>;
}

pub struct Git {
    pub(crate) path: PathBuf
}

impl Git {
    pub fn new(path: &PathBuf) -> Self {
        Self {path: PathBuf::from(path) }
    }
    fn convert_time(time: &Time) -> Tm {
        let ts = time::Timespec::new(time.seconds() + (time.offset_minutes() as i64) * 60, 0);
        time::at(ts)
    }
    fn convert_status(status: &git2::Delta) -> DeltaStatus {
        match status {
            git2::Delta::Added => {DeltaStatus::Added}
            git2::Delta::Deleted => {DeltaStatus::Deleted}
            git2::Delta::Modified => {DeltaStatus::Modified}
            git2::Delta::Renamed => {DeltaStatus::Renamed}
            git2::Delta::Copied => {DeltaStatus::Copied}
            _ => {DeltaStatus::Other}
        }
    }
}

impl Repo for Git {
    fn commits(&self) -> Vec<Commit> {
        let mut commits = vec![];
        let repo = Repository::open(&self.path).unwrap();
        if repo.is_empty().unwrap() {
            return commits;
        }
        let mut rev_walk = repo.revwalk().unwrap();
        rev_walk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL).unwrap();
        rev_walk.push_head().unwrap();
        for id in rev_walk {
            let id = id.unwrap();
            let commit = repo.find_commit(id).unwrap();
            let time = Self::convert_time(&commit.time());
            let mut c = Commit { id: commit.id().to_string(), message: commit.message().unwrap().to_string(), time, author: commit.author().to_string(), deltas: vec![] };
            let previous = if commit.parents().len() == 1 {
                let parent = commit.parent(0).unwrap();
                Some(parent.tree().unwrap())
            } else {
                None
            };
            let current = commit.tree().unwrap();
            let mut diff = repo.diff_tree_to_tree(previous.as_ref(), Some(&current), None).unwrap();
            let mut opts = DiffFindOptions::new();
            opts.break_rewrites_for_renames_only(true);
            opts.all(true);
            diff.find_similar(Some(&mut opts)).unwrap();

            for i in 0..diff.deltas().len() {
                let patch = Patch::from_diff(&diff, i).unwrap().unwrap();
                let delta = patch.delta();
                let (_context, additions, deletions) = patch.line_stats().unwrap();
                let d = Delta{
                    old_file: String::from(delta.old_file().path().unwrap().to_str().unwrap()),
                    new_file: String::from(delta.new_file().path().unwrap().to_str().unwrap()),
                    status: Self::convert_status(&delta.status()),
                    lines: (additions + deletions) as i32
                };
                c.deltas.push(d);
            }
            commits.push(c);
        }
        commits
    }
}