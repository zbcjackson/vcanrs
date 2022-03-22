mod git;
mod churn_analyzer;
mod churn_reporter;
mod application;

use crate::application::run;

fn main() {
    run();
}