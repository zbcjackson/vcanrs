mod git;
mod churn_analyzer;
mod churn_reporter;

use crate::churn_analyzer::ChurnAnalyzer;
use crate::git::{Git, Repo};


fn run() {
    let path = "../../vcanr";
    let mut churn_analyzer = ChurnAnalyzer::new(path.to_string());
    churn_analyzer.analyze();
    churn_analyzer.report();
}

fn main() {
    run();
}