use crate::{adapters::repository, domain::models};
use rusqlite::params;

pub fn list_budgets(repo: &repository::SQLiteRepository) -> Vec<models::Budget> {
    let conn = repo.conn.borrow();

    let mut stmt = conn.prepare("SELECT * from budgets").unwrap();
    let budget_itr = stmt.query_map(params![], |row| {
        Ok(models::Budget::load(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    }).unwrap();

    let loaded_budgets = budget_itr.map(|f| f.unwrap()).collect();

    loaded_budgets
}

pub fn list_transactions(budget_id: &str, repo: &repository::SQLiteRepository) -> Vec<models::Transaction> {
    let conn = repo.conn.borrow();

    let mut stmt = conn.prepare("SELECT * FROM transactions WHERE budget_id = ?1").unwrap();

    let tx_iter = stmt.query_map(params![budget_id], |row| {
        Ok(models::Transaction::load(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    }).unwrap();

    let tx: Vec<models::Transaction> = tx_iter.map(|f| f.unwrap()).collect();

    tx
}


// #[cfg(test)]
// mod tests {
//     use super::*;
// }
