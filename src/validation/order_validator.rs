use pyo3::prelude::*;
use crate::models::order::Order;

#[pyclass]
pub struct OrderValidator;

#[pymethods]
impl OrderValidator {
    #[new]
    pub fn new() -> Self {
        OrderValidator
    }

    pub fn validate(&self, order: &Order) -> PyResult<()> {
        if order.quantity <= 0.0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Order quantity must be positive"));
        }
        if order.price < 0.0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Order price must be non-negative"));
        }
        Ok(())
    }
}