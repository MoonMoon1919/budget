#![allow(dead_code)]
#![allow(unused_variables)]

use crate::domain::models;

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

    fn run(&self) -> models::BudgetManager {
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

    fn run(&self) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

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

    #[test]
    fn user_can_create_budget() {
        // Given
        let cmd = create_budget_cmd();

        // When
        let bdg = cmd.run();

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

        // When
        cmd.run();

        // Then
    }
}
