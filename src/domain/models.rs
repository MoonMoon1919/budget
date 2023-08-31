// Temporary while developing to reduce noise
#![allow(dead_code)]

use uuid::Uuid;

use std::{cell::RefCell, collections::HashMap};

#[derive(Debug)]
struct User {
    id: String,
    budgets: HashMap<String, BudgetManager>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BudgetManager {
    budget: Budget,
    transactions: RefCell<Vec<Transaction>>,
}

impl BudgetManager {
    pub fn new(budget: Budget, transactions: RefCell<Vec<Transaction>>) -> Self {
        BudgetManager {
            transactions: transactions,
            budget,
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

    pub fn budget(&self) -> &Budget {
        &self.budget
    }

    pub fn transactions(&self) -> &RefCell<Vec<Transaction>> {
        &self.transactions
    }

    pub fn add_tx(&mut self, name: String, value: f64) -> String {
        let tx = Transaction::new(name, value, String::from(self.id()));
        let txc = tx.clone().id;

        self.budget.withdraw(&tx.value);
        self.transactions.borrow_mut().push(tx);

        txc
    }

    pub fn find_tx_index(&self, id: &str) -> Result<usize, String> {
        for (i, tx) in self.transactions.borrow().iter().enumerate() {
            if &tx.id == id {
                return Ok(i);
            } else {
                continue;
            }
        }

        Err(String::from("Error, not found"))
    }

    pub fn update_tx(&mut self, id: &str, val: f64) {
        let idx = match self.find_tx_index(id) {
            Ok(i) => i,
            _ => panic!("Transaction not found"),
        };

        let mut txs = self.transactions.borrow_mut();
        let tx = txs.get_mut(idx).expect("Transaction not found");

        self.budget.deposit(&tx.value);
        tx.update_value(val);
        self.budget.withdraw(&tx.value);
    }

    pub fn remove_tx(&mut self, id: &str) {
        let idx = match self.find_tx_index(id) {
            Ok(i) => i,
            _ => panic!("Transaction not found"),
        };

        let mut txs = self.transactions.borrow_mut();
        let tx = txs.get(idx).expect("Transaction not found");

        self.budget.deposit(&tx.value);
        txs.remove(idx);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Budget {
    id: String,
    name: String,
    total: f64,
}

impl Budget {
    pub fn new(name: String, total: f64) -> Self {
        Budget {
            id: Uuid::new_v4().to_string(),
            name,
            total,
        }
    }

    pub fn load(id: String, name: String, total: f64) -> Self {
        Budget { id, name, total }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total(&self) -> &f64 {
        &self.total
    }

    fn available_funds(&self) -> f64 {
        self.total
    }

    fn can_withdraw(&self, val: &f64) -> bool {
        self.total - val > 0_f64
    }

    fn withdraw(&mut self, val: &f64) {
        if self.can_withdraw(val) {
            self.total = self.total - val;
        } else {
            // TODO: use Result instead of panic
            panic!("Insufficient funds, cannot withdraw {}", val)
        }
    }

    fn deposit(&mut self, val: &f64) {
        self.total = self.total + val;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: String,
    name: String,
    value: f64,
    budget_id: String,
}

impl Transaction {
    pub fn new(name: String, value: f64, budget_id: String) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            name,
            value,
            budget_id,
        }
    }

    pub fn load(id: String, name: String, value: f64, budget_id: String) -> Self {
        Transaction {
            id,
            name,
            value,
            budget_id,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &f64 {
        &self.value
    }

    pub fn budget_id(&self) -> &str {
        &self.budget_id
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
        let mut budgman = BudgetManager::new(budg, RefCell::new(vec![]));

        budgman.add_tx(String::from("cheeseborger"), 3.99_f64);

        assert_eq!(budgman.transactions.borrow().len(), 1);
        assert_eq!(budgman.budget.available_funds(), 196.01_f64)
    }

    #[test]
    fn budget_manager_can_remove_transaction() {
        let budg = Budget::new(String::from("my-budget"), 200.00_f64);
        let mut budgman = BudgetManager::new(budg, RefCell::new(vec![]));

        let tx_id = budgman.add_tx(String::from("cheeseborger"), 3.99_f64);
        budgman.remove_tx(&tx_id);

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
    #[should_panic]
    fn budget_cant_withdraw_money_more_than_available() {
        let mut budget = Budget::new(String::from("my-budget"), 9.00_f64);

        budget.withdraw(&10.00_f64);
    }

    #[test]
    fn budget_can_deposit_money() {
        let mut budget = Budget::new(String::from("my-budget"), 200.00_f64);

        budget.deposit(&10.00_f64);

        assert_eq!(budget.available_funds(), 210.00_f64)
    }

    #[test]
    fn tx_can_get_renamed() {
        let mut tx = Transaction::new(
            String::from("cheeseborger"),
            3.99_f64,
            String::from("abc123"),
        );

        tx.rename(String::from("cheeseburger"));

        assert_eq!(tx.name, String::from("cheeseburger"))
    }

    #[test]
    fn tx_can_have_value_updated() {
        let mut tx = Transaction::new(
            String::from("cheeseborger"),
            3.99_f64,
            String::from("abc123"),
        );

        tx.update_value(4.99_f64);

        assert_eq!(tx.value, 4.99_f64)
    }
}
