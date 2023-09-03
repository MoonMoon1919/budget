use std::env::current_dir;

use budget::cli::cli_args;
use budget::entrypoints::cli_entrypoints;
use budget::adapters::repository;

fn main() {
    let args = cli_args::parse_args();
    let pwd = current_dir().unwrap();
    let repo = repository::SQLiteRepository::new(format!("{}/budgets.db", pwd.to_string_lossy()));

    match args.commands {
        cli_args::Commands::Budgets(bargs) => {
            cli_entrypoints::handle_budget(bargs, &repo);
        }
        cli_args::Commands::Transactions(txargs) => {
            cli_entrypoints::handle_transaction(txargs, &repo);
        }
    }
}
