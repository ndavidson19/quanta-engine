use pyo3::prelude::*;
use crate::models::order::Order;

#[pyclass]
pub struct OrderExecutor;

#[pymethods]
impl OrderExecutor {
    #[new]
    pub fn new() -> Self {
        OrderExecutor
    }

    pub fn execute(&self, order: &Order) -> PyResult<String> {
        // Simplified execution logic
        Ok(format!("Executed order: {} {} of {} at ${}", 
                   if order.quantity > 0.0 { "Buy" } else { "Sell" },
                   order.quantity.abs(),
                   order.symbol,
                   order.price))
    }
}