use crate::churn_analyzer::ChurnAnalyzer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "vcanrs")]
#[clap(about = "A repo analyze CLI", long_about = None)]
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
            let mut churn_analyzer = ChurnAnalyzer::new(path.to_string());
            churn_analyzer.analyze();
            churn_analyzer.report();
        }
    }
}
