use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[pyclass]
#[derive(Clone)]
pub struct Strategy {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub user_id: String,
    #[pyo3(get)]
    pub status: StrategyStatus,
    created_at: i64,
    updated_at: i64,
}

#[pymethods]
impl Strategy {
    #[new]
    fn new(id: String, name: String, user_id: String) -> Self {
        let now = Utc::now().timestamp();
        Strategy {
            id,
            name,
            user_id,
            status: StrategyStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    #[getter]
    fn get_created_at<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDateTime> {
        PyDateTime::from_timestamp(py, self.created_at as f64, None)
    }

    #[getter]
    fn get_updated_at<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDateTime> {
        PyDateTime::from_timestamp(py, self.updated_at as f64, None)
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub enum StrategyStatus {
    Active,
    Paused,
    Stopped,
}

#[pyclass]
pub struct StrategyManager {
    strategies: HashMap<String, Strategy>,
}

#[pymethods]
impl StrategyManager {
    #[new]
    fn new() -> Self {
        StrategyManager {
            strategies: HashMap::new(),
        }
    }

    fn add_strategy(&mut self, id: String, name: String, user_id: String) -> PyResult<()> {
        let strategy = Strategy::new(id.clone(), name, user_id);
        self.strategies.insert(id, strategy);
        Ok(())
    }

    fn get_strategy(&self, id: &str) -> Option<Strategy> {
        self.strategies.get(id).cloned()
    }

    fn update_strategy_status(&mut self, id: &str, status: StrategyStatus) -> PyResult<()> {
        if let Some(strategy) = self.strategies.get_mut(id) {
            strategy.status = status;
            strategy.updated_at = Utc::now().timestamp();
            Ok(())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err("Strategy not found"))
        }
    }

    fn list_active_strategies(&self) -> Vec<Strategy> {
        self.strategies.values()
            .filter(|s| matches!(s.status, StrategyStatus::Active))
            .cloned()
            .collect()
    }
}