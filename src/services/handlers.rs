#![allow(dead_code)]

use crate::domain::models;
use crate::adapters::repository;

/// This module contains handler functions that call business logic and persistence layers
/// This module is the API for consumers, e.g., an web API or CLI that is implemented later

struct CreateBudget {
    user_id: String,
    budget_name: String,
    total: f64,
}

impl CreateBudget {
    fn new(user_id: String, budget_name: String, total: f64) -> Self {
        CreateBudget { user_id, budget_name, total }
    }

    fn run<T: repository::Repository>(&self, repo: &T) -> models::BudgetManager {
        todo!()
    }
}

struct AddTransaction {
    user_id: String,
    budget_id: String,
    name: String,
    value: f64,
}

impl AddTransaction {
    fn new(user_id: String, budget_id: String, name: String, value: f64) -> Self {
        AddTransaction {
            user_id,
            budget_id,
            name,
            value
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) {
        todo!()
    }
}

struct RemoveTransaction {
    user_id: String,
    budget_id: String,
    transaction_id: String
}

impl RemoveTransaction {
    fn new(user_id: String, budget_id: String, transaction_id: String) -> Self {
        RemoveTransaction {
            user_id,
            budget_id,
            transaction_id
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) {
        todo!()
    }
}

struct UpdateTransaction {
    user_id: String,
    budget_id: String,
    transaction_id: String,
    new_val: f64
}

impl UpdateTransaction {
    fn new(user_id: String, budget_id: String, transaction_id: String, new_val: f64) -> Self {
        UpdateTransaction { user_id,
            budget_id,
            transaction_id,
            new_val
        }
    }

    fn run<T: repository::Repository>(&self, repo: &T) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use crate::adapters::repository;

    fn user_id() -> String {
        Uuid::new_v4().to_string()
    }

    fn budget_name() -> String {
        String::from("my-budget")
    }

    fn budget_id() -> String {
        Uuid::new_v4().to_string()
    }

    fn budget_max() -> f64 {
        200.00_f64
    }

    fn transaction_id() -> String {
        Uuid::new_v4().to_string()
    }

    fn create_budget_cmd() -> CreateBudget {
        CreateBudget::new(
            user_id(),
            budget_name(),
            budget_max()
        )
    }

    fn add_transaction_cmd(budget_id: String, name: String, value: f64) -> AddTransaction {
        AddTransaction::new(
            user_id(),
            budget_id,
            name,
            value
        )
    }

    // Fake repository
    struct InMemoryRepository {
        budgets: RefCell<HashMap<String, models::BudgetManager>>
    }

    impl InMemoryRepository {
        fn new() -> Self {
            InMemoryRepository {
                budgets: RefCell::new(HashMap::new())
            }
        }
    }

    impl repository::Repository for InMemoryRepository {
        fn add(&self, item: models::BudgetManager) {
            self.budgets.borrow_mut().insert(item.id().to_string(), item);
        }

        fn get(&self, id: &str) -> models::BudgetManager {
            let budgets = self.budgets.borrow();
            let budget = budgets.get(id);

            let budg = match budget {
                Some(result) => result,
                _ => panic!("Budget not found")
            };

            budg.clone()
        }

        fn delete(&self, id: &str) {
            let mut budgets = self.budgets.borrow_mut();

            budgets.remove(id).unwrap();
        }
    }

    #[test]
    fn user_can_create_budget() {
        // Given
        let cmd = create_budget_cmd();
        let repo = InMemoryRepository::new();

        // When
        let bdg = cmd.run(&repo);

        // Then
        assert_eq!(bdg.name(), String::from("my-budget"))
    }

    #[test]
    fn user_can_add_transaction() {
        // Given
        let cmd = add_transaction_cmd(
            budget_id(),
            String::from("cheeseborger"),
            9.99_f64
        );
        let repo = InMemoryRepository::new();

        // When
        cmd.run(&repo);

        // Then
    }

    #[test]
    fn user_can_remove_transaction() {
        // Given
        let cmd = RemoveTransaction::new(
            user_id(),
            budget_id(),
            transaction_id()
        );
        let repo = InMemoryRepository::new();

        // When
        cmd.run(&repo);

        // Then
    }

    #[test]
    fn user_can_update_transaction() {
        // Given
        let cmd = UpdateTransaction::new(
            user_id(),
            budget_id(),
            transaction_id(),
            3.99_f64
        );
        let repo = InMemoryRepository::new();

        // When
        cmd.run(&repo);

        // Then
    }
}
