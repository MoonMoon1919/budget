// Temporary while developing to reduce noise
#![allow(dead_code)]
#![allow(unused_variables)]

use uuid::Uuid;

use std::collections::HashMap;

#[derive(Debug)]
struct User {
    budgets: HashMap<String, Budget>
}

#[derive(Debug)]
struct Budget {
    user_id: String,
    id: String,
    name: String,
    transactions: Vec<Transaction>
}

impl Budget {
    fn new(name: String, user_id: String) -> Self {
        Budget {
            user_id: user_id,
            id: Uuid::new_v4().to_string(),
            name: name,
            transactions: vec![]
        }
    }

    fn add_tx(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    fn find_tx_index(&self, id: &String) -> Result<usize, String> {
        for (i, tx) in self.transactions.iter().enumerate() {
            if &tx.id == id {
                return Ok(i)
            } else {
                continue;
            }
        }

        Err(String::from("Error, not found"))
    }

    fn remove_tx(&mut self, id: String) {
        let idx = match self.find_tx_index(&id) {
            Ok(i) => i,
            _ => panic!("Index not found")
        };

        self.transactions.remove(idx);
    }
}

#[derive(Debug)]
struct Transaction {
    id: String,
    name: String,
    value: f64,
}

impl Transaction {
    fn new(name: String, value: f64) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            name,
            value,
        }
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn update_value(&mut self, value: f64) {
        self.value = value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_can_add_transaction() {
        let mut budg = Budget::new(String::from("my-budget"), Uuid::new_v4().to_string());
        let tx = Transaction::new(String::from("cheeseborger"), 3.99_f64);

        budg.add_tx(tx);

        assert_eq!(budg.transactions.len(), 1)
    }

    #[test]
    fn budget_can_remove_transaction() {
        let mut budg = Budget::new(String::from("my-budget"), Uuid::new_v4().to_string());
        let tx = Transaction::new(String::from("cheeseborger"), 3.99_f64);
        let tx_id = tx.id.clone();

        budg.add_tx(tx);

        budg.remove_tx(tx_id);

        assert_eq!(budg.transactions.len(), 0)
    }
}
