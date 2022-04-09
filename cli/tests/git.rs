use test_context::{test_context, TestContext};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use assert_matches::assert_matches;
use chrono::Local;
use tempfile::{tempdir, TempDir};

extern crate vcanrs;

use vcanrs::git::{Git, Repo, DeltaStatus};


#[derive(Default)]
struct GitContext {
    #[allow(dead_code)]
    tmp_dir: Option<TempDir>,
    path: PathBuf,
    repo: Option<Git>,
}

impl GitContext {
    pub fn new() -> Self {
        let mut context = GitContext::default();
        context.init_repo();
        context
    }
    fn init_repo(&mut self) {
        self.tmp_dir = Some(tempdir().unwrap());
        let dt = Local::now();
        self.path = self.tmp_dir.as_ref().unwrap().path().join(format!("repo{}", dt.timestamp_millis()));
        fs::create_dir_all(&self.path).expect("Create repo dir error.");
        Command::new("git").arg("init").current_dir(&self.path).output().expect("Init repo error.");
        self.repo = Some(Git::new(&self.path));
    }

    fn delete_repo(&self) {
        Command::new("rm").arg("-rf").arg(&self.path).output().expect("Delete repo error.");
    }

    fn add_or_change_file(&self, file: &Path) {
        fs::write(self.path.join(file), rand::random::<i32>().to_string()).unwrap()
    }

    fn delete_file(&self, file: &Path) {
        fs::remove_file(self.path.join(file)).unwrap();
    }

    fn rename_file(&self, old_file: &Path, new_file: &Path) {
        fs::rename(self.path.join(old_file), self.path.join(new_file)).unwrap();
    }

    fn add_commit(&self) {
        Command::new("git").arg("add").arg(".").current_dir(&self.path).output().expect("Git add change error.");
        Command::new("git").arg("commit").arg("-m").arg("commit message").current_dir(&self.path).output().expect("Git add change error.");
    }
}

impl TestContext for GitContext {
    fn setup() -> GitContext {
        GitContext::new()
    }

    fn teardown(self) {
        self.delete_repo()
    }
}


#[test_context(GitContext)]
#[test]
fn empty_repo(ctx: &mut GitContext) {
    assert_eq!(ctx.repo.as_ref().unwrap().commits().is_empty(), true);
}

#[test_context(GitContext)]
#[test]
fn add_changes(ctx: &mut GitContext) {
    ctx.add_or_change_file(Path::new("a.txt"));
    ctx.add_commit();
    let commits = ctx.repo.as_ref().unwrap().commits();
    assert_eq!(commits[0].deltas[0].old_file, PathBuf::from("a.txt"));
    assert_eq!(commits[0].deltas[0].new_file, PathBuf::from("a.txt"));
    assert_matches!(commits[0].deltas[0].status, DeltaStatus::Added);
    assert_eq!(commits[0].deltas[0].lines, 1);
}

#[test_context(GitContext)]
#[test]
fn modify_changes(ctx: &mut GitContext) {
    ctx.add_or_change_file(Path::new("a.txt"));
    ctx.add_commit();
    ctx.add_or_change_file(Path::new("a.txt"));
    ctx.add_commit();
    let commits = ctx.repo.as_ref().unwrap().commits();
    assert_eq!(commits[1].deltas[0].old_file, PathBuf::from("a.txt"));
    assert_eq!(commits[1].deltas[0].new_file, PathBuf::from("a.txt"));
    assert_matches!(commits[1].deltas[0].status, DeltaStatus::Modified);
    assert_eq!(commits[1].deltas[0].lines, 2);
}

#[test_context(GitContext)]
#[test]
fn delete_changes(ctx: &mut GitContext) {
    ctx.add_or_change_file(Path::new("a.txt"));
    ctx.add_commit();
    ctx.delete_file(Path::new("a.txt"));
    ctx.add_commit();
    let commits = ctx.repo.as_ref().unwrap().commits();
    assert_eq!(commits[1].deltas[0].old_file, PathBuf::from("a.txt"));
    assert_eq!(commits[1].deltas[0].new_file, PathBuf::from("a.txt"));
    assert_matches!(commits[1].deltas[0].status, DeltaStatus::Deleted);
    assert_eq!(commits[1].deltas[0].lines, 1);
}

#[test_context(GitContext)]
#[test]
fn rename_changes(ctx: &mut GitContext) {
    ctx.add_or_change_file(Path::new("a.txt"));
    ctx.add_commit();
    ctx.rename_file(Path::new("a.txt"), Path::new("b.txt"));
    ctx.add_commit();
    let commits = ctx.repo.as_ref().unwrap().commits();
    assert_eq!(commits[1].deltas[0].old_file, PathBuf::from("a.txt"));
    assert_eq!(commits[1].deltas[0].new_file, PathBuf::from("b.txt"));
    assert_matches!(commits[1].deltas[0].status, DeltaStatus::Renamed);
    assert_eq!(commits[1].deltas[0].lines, 0);
}
