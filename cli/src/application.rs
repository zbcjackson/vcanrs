use crate::churn_analyzer::ChurnAnalyzer;

use clap::{Parser, Subcommand};
use crate::churn_reporter::ChurnReporter;
use crate::git::Git;

#[derive(Parser)]
#[clap(name = "vcanrs")]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Churn {
        #[clap(default_value=".")]
        path: String,
    },
}

pub fn run() {
    let args = Cli::parse();
    match &args.command {
        Commands::Churn {path} => {
            let repo = Box::new(Git::new(path.to_string()));
            let reporter = Box::new(ChurnReporter::new());
            let mut churn_analyzer = ChurnAnalyzer::new(repo, reporter);
            churn_analyzer.analyze();
            churn_analyzer.report();
        }
    }
}
