use crate::cli::cli_args;
use crate::adapters::repository;
use crate::services::handlers;

pub fn handle_budget<T: repository::Repository>(args: cli_args::BudgetArgs, repo: &T) {
    match args.commands {
        cli_args::BudgetCommands::Add { name, total } => {
            let cmd = handlers::CreateBudget::new(name, total);
            let result = cmd.run(repo);
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

pub fn handle_transaction<T: repository::Repository>(args: cli_args::TransactionArgs, repo: &T) {
    match args.commands {
        cli_args::TransactionCommands::Add { budget_id, name, amount } => {
            let cmd = handlers::AddTransaction::new(budget_id, name, amount);
            let result = cmd.run(repo);
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        cli_args::TransactionCommands::List => {
            todo!()
        }
        cli_args::TransactionCommands::Remove { budget_id, id } => {
            let cmd = handlers::RemoveTransaction::new(budget_id, id);
            cmd.run(repo);
        }
        cli_args::TransactionCommands::Update { budget_id, id, amount } => {
            let cmd = handlers::UpdateTransaction::new(budget_id, id, amount);
            cmd.run(repo);
        }
    }
}
