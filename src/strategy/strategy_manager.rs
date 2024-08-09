use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::models::user::User;
use crate::strategy::strategy::{StrategyWrapper, StrategyStatus};

#[pyclass]
pub struct StrategyManager {
    users: RwLock<HashMap<String, Arc<User>>>,
    strategies: RwLock<HashMap<String, Arc<StrategyWrapper>>>,
}

#[pymethods]
impl StrategyManager {
    #[new]
    pub fn new() -> Self {
        StrategyManager {
            users: RwLock::new(HashMap::new()),
            strategies: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_user(&self, id: String, name: String, broker_api: PyObject) -> PyResult<()> {
        let user = Arc::new(User::new(id.clone(), name, broker_api));
        self.users.write().unwrap().insert(id, user);
        Ok(())
    }

    pub fn add_strategy(&self, id: String, name: String, user_id: String, strategy: PyObject) -> PyResult<()> {
        let users = self.users.read().unwrap();
        let user = users.get(&user_id).ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("User not found"))?;
        let wrapper = Arc::new(StrategyWrapper::new(
            id.clone(),
            name,
            user_id,
            StrategyStatus::Active,
            strategy,
            (**user).clone(), // Clone the User inside the Arc
        ));
        self.strategies.write().unwrap().insert(id, wrapper);
        Ok(())
    }

    pub fn get_strategy(&self, id: &str) -> Option<StrategyWrapper> {
        self.strategies.read().unwrap().get(id).map(|arc| (**arc).clone())
    }
    
    pub fn update_strategy_status(&self, id: &str, status: StrategyStatus) -> PyResult<()> {
        let mut strategies = self.strategies.write().unwrap();
        if let Some(strategy) = strategies.get_mut(id) {
            if let Some(strategy_mut) = Arc::get_mut(strategy) {
                strategy_mut.status = status;
                Ok(())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get mutable reference to strategy"))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Strategy not found"))
        }
    }

    pub fn list_active_strategies(&self) -> Vec<StrategyWrapper> {
        self.strategies.read().unwrap().values()
            .filter(|s| matches!(s.status, StrategyStatus::Active))
            .map(|arc| (**arc).clone())
            .collect()
    }

    pub fn list_user_strategies(&self, user_id: &str) -> Vec<StrategyWrapper> {
        self.strategies.read().unwrap().values()
            .filter(|s| s.user_id == user_id)
            .map(|arc| (**arc).clone())
            .collect()
    }
}
