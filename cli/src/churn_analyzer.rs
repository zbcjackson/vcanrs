use std::collections::HashMap;
use crate::git::{DeltaStatus, Repo};
use crate::churn_reporter::Reporter;

pub struct ChurnAnalyzer<'a> {
    repo: &'a dyn Repo,
    reporter: &'a dyn Reporter,
    stat: HashMap<String, i32>,
}

impl <'a> ChurnAnalyzer<'a> {
    pub fn new(repo: &'a dyn Repo, reporter: &'a dyn Reporter) -> ChurnAnalyzer<'a> {
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
    use crate::git::MockRepo;
    use crate::churn_reporter::MockReporter;

    #[test]
    fn show_empty_stat_when_no_commits() {
        let mut repo = MockRepo::new();
        repo.expect_commits().returning(|| {vec![]});
        let mut reporter = MockReporter::new();
        reporter.expect_report().withf(|stat: &HashMap<String, i32>| stat.is_empty()).return_const(());
        let mut analyzer = ChurnAnalyzer::new(&repo, &reporter);

        analyzer.analyze();
        analyzer.report();
    }
}