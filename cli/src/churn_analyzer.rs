use mockall_double::double;
use std::collections::HashMap;
use crate::git::{DeltaStatus, Repo};
#[double]
use crate::churn_reporter::Reporter;

pub struct ChurnAnalyzer {
    repo: Box<dyn Repo>,
    reporter: Box<dyn Reporter>,
    stat: HashMap<String, i32>,
}

impl ChurnAnalyzer {
    pub fn new(repo: Box<dyn Repo>, reporter: Box<dyn Reporter>) -> ChurnAnalyzer {
        ChurnAnalyzer {
            repo,
            reporter,
            stat: Default::default(),
        }
    }
    pub fn analyze(&mut self) {
        let commits = &self.repo.commits();
        for commit in commits {
            for delta in &commit.deltas {
                match delta.status {
                    DeltaStatus::Deleted => { let _ = &self.stat.remove(&delta.old_file); }
                    DeltaStatus::Renamed => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, mock, predicate::*};
    use crate::git::{Git, Repo, Commit};
    use crate::churn_reporter::ChurnReporter;

    mock! {
        pub Git {
        }
        impl Repo for Git {
            fn commits(&self) -> Vec<Commit>;
        }
    }
    #[test]
    fn show_empty_stat_when_no_commits() {
        // let repo = Git{ path: "".to_string() };
        // let reporter = ChurnReporter::new();
        // let mut analyzer = ChurnAnalyzer::new_with(repo, reporter);
        // analyzer.analyze();
    }
}