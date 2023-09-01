use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[command(name = "budget")]
#[command(about = "Simple CLI app for creating a budget", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Budget(BudgetArgs),
    Transaction(TransactionArgs)
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BudgetArgs {
    #[command(subcommand)]
    pub commands: BudgetCommands
}

#[derive(Debug, Subcommand)]
pub enum BudgetCommands {
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        total: f64
    },
    List,
    Delete {
        #[arg(short, long)]
        id: String
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TransactionArgs {
    #[command(subcommand)]
    pub commands: TransactionCommands
}

#[derive(Debug, Subcommand)]
pub enum TransactionCommands {
    Add {
        #[arg(short, long)]
        budget_id: String,

        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        amount: f64
    },
    List,
    Update {
        #[arg(short, long)]
        budget_id: String,

        #[arg(short, long)]
        id: String,

        #[arg(short, long)]
        amount: f64
    },
    Remove {
        #[arg(short, long)]
        budget_id: String,

        #[arg(short, long)]
        id: String,
    }
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
