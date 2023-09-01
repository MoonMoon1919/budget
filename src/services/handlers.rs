#![allow(dead_code)]

use std::cell::RefCell;

use crate::adapters::repository;
use crate::domain::models;

/// This module contains handler functions that call business logic and persistence layers
/// This module is the API for consumers, e.g., an web API or CLI that is implemented later

struct CreateBudget {
    budget_name: String,
    total: f64,
}

impl CreateBudget {
    fn new(budget_name: String, total: f64) -> Self {
        CreateBudget {
            budget_name,
            total,
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) -> models::BudgetManager {
        let budget = models::Budget::new(self.budget_name.clone(), self.total);
        let budget_manager = models::BudgetManager::new(budget, RefCell::new(vec![]));

        repo.add(&budget_manager);

        budget_manager
    }
}

struct AddTransaction {
    budget_id: String,
    name: String,
    value: f64,
}

impl AddTransaction {
    fn new(budget_id: String, name: String, value: f64) -> Self {
        AddTransaction {
            budget_id,
            name,
            value,
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) -> String {
        let mut budget_manager = repo.get(&self.budget_id);

        let tx_id = budget_manager.add_tx(self.name.clone(), self.value);

        repo.add(&budget_manager);

        tx_id
    }
}

struct RemoveTransaction {
    budget_id: String,
    transaction_id: String,
}

impl RemoveTransaction {
    fn new(budget_id: String, transaction_id: String) -> Self {
        RemoveTransaction {
            budget_id,
            transaction_id,
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) {
        let mut budget_manager = repo.get(&self.budget_id);

        budget_manager.remove_tx(&self.transaction_id);

        repo.add(&budget_manager)
    }
}

struct UpdateTransaction {
    budget_id: String,
    transaction_id: String,
    new_val: f64,
}

impl UpdateTransaction {
    fn new(budget_id: String, transaction_id: String, new_val: f64) -> Self {
        UpdateTransaction {
            budget_id,
            transaction_id,
            new_val,
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) {
        let mut budget_manager = repo.get(&self.budget_id);

        budget_manager.update_tx(&self.transaction_id, self.new_val);

        repo.add(&budget_manager);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::repository::{self, Repository};
    use std::cell::RefCell;
    use std::collections::HashMap;

    fn budget_name() -> String {
        String::from("my-budget")
    }

    fn budget_max() -> f64 {
        200.00_f64
    }

    fn make_empty_budget_manager() -> models::BudgetManager {
        models::BudgetManager::new(
            models::Budget::new(budget_name(), budget_max()),
            RefCell::new(vec![]),
        )
    }

    // Fake repository
    struct InMemoryRepository {
        budgets: RefCell<HashMap<String, models::BudgetManager>>,
    }

    impl InMemoryRepository {
        fn new() -> Self {
            InMemoryRepository {
                budgets: RefCell::new(HashMap::new()),
            }
        }
    }

    impl repository::Repository for InMemoryRepository {
        fn add(&self, item: &models::BudgetManager) {
            self.budgets
                .borrow_mut()
                .insert(item.id().to_string(), item.clone());
        }

        fn get(&self, id: &str) -> models::BudgetManager {
            let budgets = self.budgets.borrow();
            let budget = budgets.get(id);

            let budg = match budget {
                Some(result) => result.clone(),
                _ => panic!("Budget not found"),
            };

            budg
        }

        fn delete(&self, id: &str) {
            let mut budgets = self.budgets.borrow_mut();

            budgets.remove(id).unwrap();
        }
    }

    #[test]
    fn user_can_create_budget() {
        // Given
        let cmd = CreateBudget::new(budget_name(), budget_max());
        let repo = InMemoryRepository::new();

        // When
        let bdg = cmd.run(&repo);

        // Then
        // The budget was created with the expected name and total
        assert_eq!(bdg.name(), budget_name());
        assert_eq!(bdg.available_funds(), budget_max());

        // We can retrieve the budget from the repository
        // and the value is equal to the value return from `run()`
        assert_eq!(repo.get(bdg.id()), bdg);
    }

    #[test]
    fn user_can_add_transaction() {
        // Given
        // Set up data required to run the test
        let budget_manager = make_empty_budget_manager();
        let repo = InMemoryRepository::new();
        repo.add(&budget_manager);

        // Set up the command
        let cmd = AddTransaction::new(
            budget_manager.id().to_string(),
            String::from("cheeseborger"),
            9.99_f64,
        );

        // When
        cmd.run(&repo);

        // Then
        let bm = repo.get(budget_manager.id());
        assert_eq!(bm.available_funds(), 190.01_f64);
    }

    #[test]
    fn user_can_remove_transaction() {
        // Given
        // Set up data required to run the test
        let mut budget_manager = make_empty_budget_manager();
        let transaction_id = budget_manager.add_tx(String::from("cheeseborger"), 3.99_f64);

        let repo = InMemoryRepository::new();
        repo.add(&budget_manager);

        // Set up the command we're going to test!
        let cmd =
            RemoveTransaction::new(budget_manager.id().to_string(), transaction_id);

        // When
        cmd.run(&repo);

        // Then
        let bm = repo.get(budget_manager.id());
        assert_eq!(bm.available_funds(), budget_max());
    }

    #[test]
    fn user_can_update_transaction() {
        // Given
        // Set up data required to run the test
        let mut budget_manager = make_empty_budget_manager();
        let transaction_id = budget_manager.add_tx(String::from("cheeseborger"), 3.99_f64);

        let repo = InMemoryRepository::new();
        repo.add(&budget_manager);

        // Set up the command we're going to test
        let cmd = UpdateTransaction::new(
            budget_manager.id().to_string(),
            transaction_id,
            4.99_f64,
        );

        // When
        cmd.run(&repo);

        // Then
        let bm = repo.get(budget_manager.id());
        assert_eq!(bm.available_funds(), 195.01_f64);
    }
}
