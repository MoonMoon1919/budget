#![allow(dead_code)]

use std::cell::RefCell;

use rusqlite::{Connection, params};

use crate::domain::models;

pub trait Repository {
    fn add(&self, #[allow(unused)] item: &models::BudgetManager) {}
    fn get(&self, #[allow(unused)] id: &str) -> models::BudgetManager {
        todo!("get must be implemented by implementer")
    }
    fn delete(&self, #[allow(unused)] id: &str) {}
}

struct SQLiteRepository {
    conn: Connection
}

impl SQLiteRepository {
    pub fn new(filename: String) -> Self {
        let db = Connection::open(filename);

        let conn = match db {
            Ok(conn) => conn,
            _ => panic!("Error opening db")
        };

        SQLiteRepository { conn }
    }
}

impl Repository for SQLiteRepository {
    fn add(&self, #[allow(unused)] item: &models::BudgetManager) {}

    fn get(&self, #[allow(unused)] id: &str) -> models::BudgetManager {
        // Get the budget - it sucks to do this in two queries
        // But i don't feel like writing data mapping logic right now
        let mut budget_statement = self.conn.prepare("SELECT * FROM budgets where id = ?1").unwrap();

        let budget = budget_statement.query_row(params![id], |row| {
            let id: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let total: f64 = row.get(2).unwrap();

            let budget = models::Budget::load(
                id,
                name,
                total,
            );

            Ok(budget)
        }).unwrap();

        // Get all the transactions - refactor later to use a join and get all the data in a single go
        // This is fine for quick hacking...
        let mut statement = self.conn.prepare(
            "SELECT * FROM transactions
                WHERE transaction.budget_id = ?1",
        ).unwrap();

        let row_iter = statement.query_map(params![id], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let value: f64 = row.get(2)?;
            let budget_id: String = row.get(3)?;

            let tx = models::Transaction::load(
                id,
                name,
                value,
                budget_id
            );

            Ok(tx)
        }).unwrap();

        let tx: Vec<models::Transaction> = row_iter.map(|f| f.unwrap()).collect();

        models::BudgetManager::new(budget, RefCell::new(tx))

    }

    fn delete(&self, #[allow(unused)] id: &str) {}
}
