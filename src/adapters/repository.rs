use crate::domain::models;

pub trait Repository {
    fn add(&self, #[allow(unused)] item: models::BudgetManager) {}
    fn get(&self, #[allow(unused)] id: &str) -> models::BudgetManager {
        todo!("get must be implemented by implementer")
    }
    fn delete(&self, #[allow(unused)] id: &str) {}
}
