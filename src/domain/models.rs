// Temporary while developing to reduce noise
#![allow(dead_code)]
#![allow(unused_variables)]

use uuid::Uuid;

use std::{collections::HashMap, cell::RefCell};

#[derive(Debug)]
struct User {
    id: String,
    budgets: HashMap<String, BudgetManager>
}

#[derive(Debug)]
pub struct BudgetManager {
    budget: Budget,
    transactions: RefCell<Vec<Transaction>>
}

impl BudgetManager {
    pub fn new(budget: Budget) -> Self {
        BudgetManager {
            transactions: RefCell::new(vec![]),
            budget
        }
    }

    pub fn available_funds(&self) -> f64 {
        self.budget.available_funds()
    }

    pub fn name(&self) -> &str {
        &self.budget.name
    }

    pub fn id(&self) -> &str {
        &self.budget.id
    }

    pub fn add_tx(&mut self, name: String, value: f64) -> String {
        let tx = Transaction::new(name, value, String::from(self.id()));
        let txc = tx.clone().id;

        self.budget.withdraw(&tx.value);
        self.transactions.borrow_mut().push(tx);

        txc
    }

    pub fn find_tx_index(&self, id: &String) -> Result<usize, String> {
        for (i, tx) in self.transactions.borrow().iter().enumerate() {
            if &tx.id == id {
                return Ok(i)
            } else {
                continue;
            }
        }

        Err(String::from("Error, not found"))
    }

    pub fn update_tx(&mut self, id: String, val: f64) {
        let idx = match self.find_tx_index(&id) {
            Ok(i) => i,
            _ => panic!("Transaction not found")
        };

        let mut txs = self.transactions.borrow_mut();
        let tx = txs.get_mut(idx).expect("Transaction not found");

        self.budget.deposit(&tx.value);
        tx.update_value(val);
        self.budget.withdraw(&tx.value);
    }


    pub fn remove_tx(&mut self, id: String) {
        let idx = match self.find_tx_index(&id) {
            Ok(i) => i,
            _ => panic!("Transaction not found")
        };

        let mut txs = self.transactions.borrow_mut();
        let tx = txs.get(idx).expect("Transaction not found");

        self.budget.deposit(&tx.value);
        txs.remove(idx);
    }
}

#[derive(Debug, Clone)]
pub struct Budget {
    id: String,
    name: String,
    total: f64,
}

impl Budget {
    fn new(name: String, total: f64) -> Self {
        Budget {
            id: Uuid::new_v4().to_string(),
            name,
            total,
        }
    }

    fn available_funds(&self) -> f64 {
        self.total
    }

    fn withdraw(&mut self, val: &f64) {
        self.total = self.total - val;
    }

    fn deposit(&mut self, val: &f64) {
        self.total = self.total + val;
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    id: String,
    name: String,
    value: f64,
    budget_id: String,
}

impl Transaction {
    fn new(name: String, value: f64, budget_id: String) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            name,
            value,
            budget_id,
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
    fn budget_manager_can_add_transaction() {
        let budg = Budget::new(String::from("my-budget"), 200.00_f64);
        let mut budgman = BudgetManager::new(budg);

        budgman.add_tx(String::from("cheeseborger"), 3.99_f64);

        assert_eq!(budgman.transactions.borrow().len(), 1);
        assert_eq!(budgman.budget.available_funds(), 196.01_f64)
    }

    #[test]
    fn budget_manager_can_remove_transaction() {
        let budg = Budget::new(String::from("my-budget"), 200.00_f64);
        let mut budgman = BudgetManager::new(budg);

        let tx_id = budgman.add_tx(String::from("cheeseborger"), 3.99_f64);
        budgman.remove_tx(tx_id);

        assert_eq!(budgman.transactions.borrow().len(), 0);
        assert_eq!(budgman.available_funds(), 200.00_f64);
    }

    #[test]
    fn budget_can_withdraw_money() {
        let mut budget = Budget::new(String::from("my-budget"), 200.00_f64);

        budget.withdraw(&10.00_f64);

        assert_eq!(budget.available_funds(), 190.00_f64)
    }

    #[test]
    fn budget_can_deposit_money() {
        let mut budget = Budget::new(String::from("my-budget"), 200.00_f64);

        budget.deposit(&10.00_f64);

        assert_eq!(budget.available_funds(), 210.00_f64)
    }

    #[test]
    fn tx_can_get_renamed() {
        let mut tx = Transaction::new(String::from("cheeseborger"), 3.99_f64, String::from("abc123"));

        tx.rename(String::from("cheeseburger"));

        assert_eq!(tx.name, String::from("cheeseburger"))
    }

    #[test]
    fn tx_can_have_value_updated() {
        let mut tx = Transaction::new(String::from("cheeseborger"), 3.99_f64, String::from("abc123"));

        tx.update_value(4.99_f64);

        assert_eq!(tx.value, 4.99_f64)
    }
}
