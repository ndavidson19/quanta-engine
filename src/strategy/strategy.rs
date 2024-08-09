use pyo3::prelude::*;
use std::sync::Arc;
use crate::models::user::User;

#[pyclass]
#[derive(Clone, Copy)]
pub enum StrategyStatus {
    Active,
    Paused,
    Stopped,
}

#[pyclass]
#[derive(Clone)]  // Derive Clone for StrategyWrapper
pub struct StrategyWrapper {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub user_id: String,
    #[pyo3(get)]
    pub status: StrategyStatus,
    strategy: PyObject,
    user: Arc<User>,
}

#[pymethods]
impl StrategyWrapper {
    #[new]
    pub fn new(id: String, name: String, user_id: String, status: StrategyStatus, strategy: PyObject, user: User) -> Self {
        StrategyWrapper {
            id,
            name,
            user_id,
            status,
            strategy,
            user: Arc::new(user),
        }
    }

    pub fn get_strategy(&self) -> PyObject {
        self.strategy.clone()
    }

    pub fn get_broker_api(&self) -> PyObject {
        self.user.get_broker_api()
    }
}