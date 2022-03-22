use std::collections::HashMap;
use git2::Delta;
use crate::git::{Git, Repo};
use crate::churn_reporter::ChurnReporter;

pub struct ChurnAnalyzer {
    repo: Git,
    reporter: ChurnReporter,
    stat: HashMap<String, i32>,
}

impl ChurnAnalyzer {
    pub fn new(path: String) -> ChurnAnalyzer {
        ChurnAnalyzer {
            repo: Git {path},
            reporter: ChurnReporter {},
            stat: Default::default()
        }
    }
    pub fn analyze(&mut self) {
        let commits = &self.repo.commits();
        for commit in commits {
            for delta in &commit.deltas {
                match delta.status {
                    Delta::Deleted => {let _ = &self.stat.remove(&delta.old_file);}
                    Delta::Renamed => {
                        self.stat.insert(delta.new_file.to_string(), *self.stat.get(&delta.old_file).unwrap());
                        if delta.lines > 0 {
                            *self.stat.get_mut(&delta.new_file).unwrap() += 1;
                        }
                        self.stat.remove(&delta.old_file);
                    }
                    _ => {
                        *self.stat.entry(delta.old_file.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    pub fn report(&self) {
        self.reporter.report(&self.stat);
    }
}