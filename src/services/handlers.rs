#![allow(dead_code)]
#![allow(unused_variables)]

use crate::domain::models;

/// This module contains handler functions that call business logic and persistence layers
/// This module is the API for consumers, e.g., an web API or CLI that is implemented later

struct CreateBudgetCommand {
    user_id: String,
    budget_name: String,
    total: f64,
}

impl CreateBudgetCommand {
    fn new(user_id: String, budget_name: String, total: f64) -> Self {
        CreateBudgetCommand { user_id, budget_name, total }
    }

    fn run(&self) -> models::BudgetManager {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn user_can_create_budget() {
        // Given
        let user_id = Uuid::new_v4().to_string();
        let budget_name = String::from("my-budget");
        let total = 200.00_f64;
        let cmd = CreateBudgetCommand::new(user_id, budget_name, total);

        // When
        let bdg = cmd.run();

        // Then
        assert_eq!(bdg.name(), String::from("my-budget"))
    }
}
