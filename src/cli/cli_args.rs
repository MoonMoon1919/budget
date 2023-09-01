use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[command(name = "budget")]
#[command(about = "Simple CLI app for creating a budget", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Budget(BudgetArgs),
    Transaction(TransactionArgs)
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct BudgetArgs {
    #[command(subcommand)]
    commands: BudgetCommands
}

#[derive(Debug, Subcommand)]
enum BudgetCommands {
    Add {
        name: String,
        total: f64
    },
    List,
    Delete,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct TransactionArgs {
    #[command(subcommand)]
    commands: TransactionCommands
}

#[derive(Debug, Subcommand)]
enum TransactionCommands {
    Add {
        budget_id: String,
        name: String,
        value: f64
    },
    List,
    Update {
        budget_id: String,
        id: String,
        value: f64
    },
    Remove {
        budget_id: String,
        id: String,
    }
}

pub fn parse_args() {
    let args = Cli::parse();

    println!("{:?}", args);
}
