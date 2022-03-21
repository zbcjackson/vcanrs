use git2::{Repository, Time};

pub struct Commit {
    pub(crate) id: String,
    pub(crate) message: String,
    pub(crate) time: Time,
    pub(crate) author: String
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
        rev_walk.set_sorting(git2::Sort::REVERSE).unwrap();
        rev_walk.push_head().unwrap();
        for id in rev_walk {
            let id = id.unwrap();
            let commit = repo.find_commit(id).unwrap();
            commits.push(Commit {id: commit.id().to_string(), message: commit.message().unwrap().to_string(), time: commit.time(), author: commit.author().to_string()});
        }
        commits
    }
}