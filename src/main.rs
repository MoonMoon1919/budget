use std::env::current_dir;

use budget::cli::cli_args;
use budget::services::handlers;
use budget::adapters::repository;

fn main() {
    let args = cli_args::parse_args();
    let pwd = current_dir().unwrap();
    let repo = repository::SQLiteRepository::new(format!("{}/budgets.db", pwd.to_string_lossy()));

    match args.commands {
        cli_args::Commands::Budget(bargs) => {
            match bargs.commands {
                cli_args::BudgetCommands::Add { name, total } => {
                    let cmd = handlers::CreateBudget::new(name, total);
                    let result = cmd.run(&repo);
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                cli_args::BudgetCommands::List => {
                    todo!()
                }
                cli_args::BudgetCommands::Delete { id } => {
                    println!("{}", id);
                    todo!()
                }
            }
        }
        cli_args::Commands::Transaction(txargs) => {
            match txargs.commands {
                cli_args::TransactionCommands::Add { budget_id, name, amount } => {
                    let cmd = handlers::AddTransaction::new(budget_id, name, amount);
                    cmd.run(&repo);
                }
                cli_args::TransactionCommands::List => {
                    todo!()
                }
                cli_args::TransactionCommands::Remove { budget_id, id } => {
                    let cmd = handlers::RemoveTransaction::new(budget_id, id);
                    cmd.run(&repo);
                }
                cli_args::TransactionCommands::Update { budget_id, id, amount } => {
                    let cmd = handlers::UpdateTransaction::new(budget_id, id, amount);
                    cmd.run(&repo);
                }
            }
        }
    }
}
