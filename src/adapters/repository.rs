#![allow(dead_code)]

use std::cell::{Ref, RefCell};

use rusqlite::{params, Connection, Transaction};

use crate::domain::models;

pub trait Repository {
    fn add(&self, #[allow(unused)] item: &models::BudgetManager) {}
    fn get(&self, #[allow(unused)] id: &str) -> models::BudgetManager {
        todo!("get must be implemented by implementer")
    }
    fn delete(&self, #[allow(unused)] id: &str) {}
}

pub struct SQLiteRepository {
    conn: RefCell<Connection>,
}

impl SQLiteRepository {
    pub fn new(filename: String) -> Self {
        let db = Connection::open(filename);

        let conn = match db {
            Ok(conn) => conn,
            _ => panic!("Error opening db"),
        };

        SQLiteRepository {
            conn: RefCell::new(conn),
        }
    }
}

fn insert_transactions<'a>(tx: &'a Transaction, transactions: Ref<Vec<models::Transaction>>) {
    let mut statement = tx
        .prepare(
            "INSERT INTO transactions
        (id, name, value, budget_id) VALUES
        (?1, ?2, ?3, ?4)",
        )
        .unwrap();

    for tx in transactions.iter() {
        statement
            .execute(params![tx.id(), tx.name(), tx.value(), tx.budget_id()])
            .unwrap();
    }
}

fn insert_budget<'a>(tx: &'a Transaction, budget: &models::Budget) {
    let mut statement = tx
        .prepare(
            "INSERT or IGNORE INTO budgets (id, name, total) VALUES (?1, ?2, ?3)",
        )
        .unwrap();

    statement
        .execute(params![budget.id(), budget.name(), budget.total()])
        .unwrap();
}

impl Repository for SQLiteRepository {
    fn add(&self, item: &models::BudgetManager) {
        let mut conn = self.conn.borrow_mut();

        let tx = conn.transaction().unwrap();

        insert_budget(&tx, item.budget());
        insert_transactions(&tx, item.transactions().borrow());

        let result = tx.commit();
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

    fn get(&self, id: &str) -> models::BudgetManager {
        // Get the budget - it sucks to do this in two queries
        // But i don't feel like writing data mapping logic right now
        let conn = self.conn.borrow();

        let mut budget_statement = conn.prepare("SELECT * FROM budgets where id = ?1").unwrap();

        let budget = budget_statement
            .query_row(params![id], |row| {
                let id: String = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                let total: f64 = row.get(2).unwrap();

                let budget = models::Budget::load(id, name, total);

                Ok(budget)
            })
            .unwrap();

        // Get all the transactions - refactor later to use a join and get all the data in a single go
        // This is fine for quick hacking...
        let mut statement = conn
            .prepare(
                "SELECT * FROM transactions
                WHERE budget_id = ?1",
            )
            .unwrap();

        let row_iter = statement
            .query_map(params![id], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let value: f64 = row.get(2)?;
                let budget_id: String = row.get(3)?;

                let tx = models::Transaction::load(id, name, value, budget_id);

                Ok(tx)
            })
            .unwrap();

        let tx: Vec<models::Transaction> = row_iter.map(|f| f.unwrap()).collect();

        models::BudgetManager::new(budget, RefCell::new(tx))
    }

    fn delete(&self, id: &str) {
        let conn = self.conn.borrow();
        let mut statement = conn.prepare("DELETE FROM budgets WHERE id = ?1").unwrap();
        statement.execute(params![id]).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tables(repo: &SQLiteRepository) {
        let conn = repo.conn.borrow();

        conn.execute("PRAGMA foreign_keys = ON", ()).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS budgets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            total REAL
        )", ()).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            value REAL,
            budget_id TEXT NOT NULL,
            FOREIGN KEY (budget_id)
               REFERENCES budgets (id)
               ON DELETE CASCADE
        )", ()).unwrap();
    }

    fn drop_tables(repo: &SQLiteRepository) {
        let conn = repo.conn.borrow();

        conn.execute("DROP TABLE budgets", ()).unwrap();
        conn.execute("DROP TABLE transactions", ()).unwrap();
    }

    fn build_budget_manager_with_tx() -> models::BudgetManager {
        let budget = models::Budget::new(String::from("my-budget"), 200.00_f64);
        let mut bm = models::BudgetManager::new(budget, RefCell::new(vec![]));
        bm.add_tx(String::from("cheeseborger"), 3.99_f64);

        bm
    }

    #[test]
    fn can_add_retrieve_budget_manager_aggregate() {
        // Given
        let repo = SQLiteRepository::new(String::from("budgets-int.db"));
        create_tables(&repo);

        let bm = build_budget_manager_with_tx();

        // When
        repo.add(&bm);

        // Then
        let retrieved_bm = repo.get(bm.id());

        assert_eq!(bm.available_funds(), retrieved_bm.available_funds());
        assert_eq!(bm.transactions(), retrieved_bm.transactions());

        // Drop tables
        drop_tables(&repo);
    }

    #[test]
    #[should_panic]
    fn can_delete_budget_manager_aggregate() {
        // Given
        let repo = SQLiteRepository::new(String::from("budgets-int.db"));
        create_tables(&repo);

        let bm = build_budget_manager_with_tx();
        repo.add(&bm);

        // When
        repo.delete(&bm.id());

        // Then
        repo.get(&bm.id());

        // Cleanup
        drop_tables(&repo);
    }
}
