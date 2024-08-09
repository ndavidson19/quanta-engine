use pyo3::prelude::*;

mod models;
mod validation;
mod execution;
mod risk;
mod strategy;

use models::order::{Order, OrderType};
use validation::order_validator::OrderValidator;
use execution::executor::OrderExecutor;
use risk::risk_validator::RiskValidator;
use strategy::strategy_manager::{StrategyManager, Strategy, StrategyStatus};

#[pymodule]
fn quanta_engine(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Order>()?;
    m.add_class::<OrderType>()?;
    m.add_class::<OrderValidator>()?;
    m.add_class::<OrderExecutor>()?;
    m.add_class::<RiskValidator>()?;
    m.add_class::<StrategyManager>()?;
    m.add_class::<Strategy>()?;
    m.add_class::<StrategyStatus>()?;
    
    // Import Python's datetime module
    let datetime = py.import("datetime")?;
    m.add("datetime", datetime)?;

    Ok(())
}