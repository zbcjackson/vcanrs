use test_context::{test_context, TestContext};
use std::fs;
use std::path::{PathBuf};
use std::process::Command;
use chrono::Local;
use tempfile::{tempdir, TempDir};
extern crate vcanrs;
use vcanrs::git::{Git, Repo};


#[derive(Default)]
struct GitContext {
    #[allow(dead_code)]
    tmp_dir: Option<TempDir>,
    path: PathBuf,
    repo: Option<Git>
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