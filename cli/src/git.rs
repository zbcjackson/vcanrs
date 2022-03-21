use git2::{DiffFindOptions, Repository, Time};

pub struct Delta {
    pub(crate) old_file: String,
    pub(crate) new_file: String,
    pub(crate) status: git2::Delta,
    pub(crate) lines: i32,
}

pub struct Commit {
    pub(crate) id: String,
    pub(crate) message: String,
    pub(crate) time: Time,
    pub(crate) author: String,
    pub(crate) deltas: Vec<Delta>
}

pub trait Repo {
    fn commits(&self) -> Vec<Commit>;
}

pub struct Git {
    pub(crate) path: String
}

impl Git {
}

impl Repo for Git {
    fn commits(&self) -> Vec<Commit> {
        let mut commits = vec![];
        let repo = Repository::open(&self.path).unwrap();
        let mut rev_walk = repo.revwalk().unwrap();
        rev_walk.set_sorting(git2::Sort::REVERSE | git2::Sort::TOPOLOGICAL).unwrap();
        rev_walk.push_head().unwrap();
        for id in rev_walk {
            let id = id.unwrap();
            let commit = repo.find_commit(id).unwrap();
            let mut c = Commit { id: commit.id().to_string(), message: commit.message().unwrap().to_string(), time: commit.time(), author: commit.author().to_string(), deltas: vec![] };
            let previous = if commit.parents().len() == 1 {
                let parent = commit.parent(0).unwrap();
                Some(parent.tree().unwrap())
            } else {
                None
            };
            let current = commit.tree().unwrap();
            let mut diff = repo.diff_tree_to_tree(previous.as_ref(), Some(&current), None).unwrap();
            let mut opts = DiffFindOptions::new();
            opts.break_rewrite_threshold(30);
            opts.all(true);
            diff.find_similar(Some(&mut opts)).unwrap();
            diff.deltas().for_each(|delta| {
               let d = Delta{old_file: String::from(delta.old_file().path().unwrap().to_str().unwrap()), new_file: String::from(delta.new_file().path().unwrap().to_str().unwrap()), status: delta.status(), lines: 0 };
                c.deltas.push(d);
            });
            commits.push(c);
        }
        commits
    }
}