use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, warn, error};
use crate::models::order::{Order, OrderType};

#[pyclass]
#[derive(Clone)]
pub struct ValidationRule {
    min_quantity: f64,
    max_quantity: f64,
    min_price: f64,
    max_price: f64,
}

#[pymethods]
impl ValidationRule {
    #[new]
    pub fn new(min_quantity: f64, max_quantity: f64, min_price: f64, max_price: f64) -> Self {
        ValidationRule {
            min_quantity,
            max_quantity,
            min_price,
            max_price,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct OrderValidator {
    rules: HashMap<OrderType, ValidationRule>,
    symbols: Vec<String>,
    max_order_age: Option<f64>,
}

#[pymethods]
impl OrderValidator {
    #[new]
    pub fn new() -> Self {
        OrderValidator {
            rules: HashMap::new(),
            symbols: Vec::new(),
            max_order_age: None,
        }
    }

    pub fn set_rule(&mut self, order_type: OrderType, rule: ValidationRule) -> PyResult<()> {
        let order_type_clone = order_type.clone();
        self.rules.insert(order_type, rule);
        info!("Set new rule for order type: {:?}", order_type_clone);
        Ok(())
    }

    pub fn add_symbol(&mut self, symbol: String) -> PyResult<()> {
        self.symbols.push(symbol.clone());
        info!("Added new symbol: {}", symbol);
        Ok(())
    }

    pub fn set_max_order_age(&mut self, max_age: Option<f64>) -> PyResult<()> {
        self.max_order_age = max_age;
        info!("Set max order age to: {:?}", max_age);
        Ok(())
    }

    pub fn validate(&self, order: &Order) -> PyResult<()> {
        info!("Validating order: {:?}", order);

        // Validate symbol
        if !self.symbols.contains(&order.symbol) {
            let err_msg = format!("Invalid symbol: {}", order.symbol);
            error!("{}", err_msg);
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
        }

        // Validate order type and rules
        if let Some(rule) = self.rules.get(&order.order_type) {
            if order.quantity < rule.min_quantity || order.quantity > rule.max_quantity {
                let err_msg = format!("Order quantity must be between {} and {}", rule.min_quantity, rule.max_quantity);
                error!("{}", err_msg);
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
            }

            if order.order_type != OrderType::Market && (order.price < rule.min_price || order.price > rule.max_price) {
                let err_msg = format!("Order price must be between {} and {}", rule.min_price, rule.max_price);
                error!("{}", err_msg);
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
            }
        } else {
            let err_msg = format!("No validation rule for order type: {:?}", order.order_type);
            error!("{}", err_msg);
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
        }

        // Validate order age if max_order_age is set
        if let Some(max_age) = self.max_order_age {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            if current_time - order.timestamp > max_age {
                let err_msg = "Order is too old";
                error!("{}", err_msg);
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
            }
        }

        info!("Order validation successful");
        Ok(())
    }

    pub fn validate_multiple(&self, orders: &PyList) -> PyResult<Vec<(usize, String)>> {
        info!("Starting batch validation of {} orders", orders.len());
        let mut errors = Vec::new();
        for (index, order_obj) in orders.iter().enumerate() {
            let order: Order = order_obj.extract()?;
            if let Err(e) = self.validate(&order) {
                warn!("Validation failed for order at index {}: {}", index, e);
                errors.push((index, e.to_string()));
            }
        }
        info!("Batch validation completed. {} errors found", errors.len());
        Ok(errors)
    }
}

#[pymodule]
fn your_module(_py: Python, m: &PyModule) -> PyResult<()> {
    // Initialize logging (you may need to configure this based on your project's logging setup)
    // pyo3_log::init();
    m.add_class::<OrderValidator>()?;
    m.add_class::<ValidationRule>()?;
    Ok(())
}