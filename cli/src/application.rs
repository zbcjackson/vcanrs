use std::path::PathBuf;
use crate::churn_analyzer::ChurnAnalyzer;

use clap::{Parser, Subcommand};
use crate::reporter::TableReporter;
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
            let repo = Git::new(&PathBuf::from(path));
            let reporter = TableReporter::new();
            let mut churn_analyzer = ChurnAnalyzer::new(&repo, &reporter);
            churn_analyzer.analyze();
            churn_analyzer.report();
        }
    }
}
