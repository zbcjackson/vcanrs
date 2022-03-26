use std::collections::HashMap;
use crate::git::{DeltaStatus, Repo};
use crate::churn_reporter::Reporter;

pub struct ChurnAnalyzer<'a> {
    repo: &'a dyn Repo,
    reporter: &'a dyn Reporter,
    stat: HashMap<String, i32>,
}

impl<'a> ChurnAnalyzer<'a> {
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
    use crate::git::{Commit, Delta, MockRepo};
    use crate::churn_reporter::MockReporter;

    #[test]
    fn show_empty_stat_when_no_commits() {
        verify_churn(|| vec![], |stat: &HashMap<String, i32>| assert_stat(stat, vec![]));
    }

    #[test]
    fn count_file_changes() {
        verify_churn(|| {
            vec![
                commit(vec![delta("a.txt", DeltaStatus::Added), delta("b.txt", DeltaStatus::Added)]),
                commit(vec![delta("a.txt", DeltaStatus::Modified)]),
            ]
        }, |stat: &HashMap<String, i32>| assert_stat(stat, vec![("a.txt".to_string(), 2), ("b.txt".to_string(), 1)]))
    }

    #[test]
    fn remove_file_stat_when_file_change_is_deleted() {
        verify_churn(|| {
            vec![
                commit(vec![delta("a.txt", DeltaStatus::Added), delta("b.txt", DeltaStatus::Added)]),
                commit(vec![delta("a.txt", DeltaStatus::Deleted)]),
            ]
        }, |stat: &HashMap<String, i32>| assert_stat(stat, vec![("b.txt".to_string(), 1)]));
    }

    fn verify_churn(commits: fn() -> Vec<Commit>, assert: fn(&HashMap<String, i32>) -> bool) {
        let mut repo = MockRepo::new();
        repo.expect_commits().returning(commits);
        let mut reporter = MockReporter::new();
        reporter.expect_report().withf(assert).return_const(());
        let mut analyzer = ChurnAnalyzer::new(&repo, &reporter);

        analyzer.analyze();
        analyzer.report();
    }

    fn assert_stat(stat: &HashMap<String, i32>, expected: Vec<(String, i32)>) -> bool {
        stat.eq(&expected.into_iter().collect::<HashMap<_, _>>())
    }

    fn commit(deltas: Vec<Delta>) -> Commit {
        let mut commit = Commit {
            id: "".to_string(),
            message: "".to_string(),
            time: time::now(),
            author: "".to_string(),
            deltas: vec![],
        };
        for delta in deltas {
            commit.deltas.push(delta);
        }
        commit
    }

    fn delta(file: &str, status: DeltaStatus) -> Delta {
        Delta {
            old_file: file.to_string(),
            new_file: file.to_string(),
            status,
            lines: 0,
        }
    }
}