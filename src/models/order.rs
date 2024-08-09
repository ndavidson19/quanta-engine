use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3::types::PyDateTime;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[pyo3(get, set)]
    pub symbol: String,
    #[pyo3(get, set)]
    pub quantity: f64,
    #[pyo3(get, set)]
    pub order_type: OrderType,
    #[pyo3(get, set)]
    pub price: f64,
    timestamp: f64,  // Unix timestamp as f64, no getter/setter attribute
}

#[pymethods]
impl Order {
    #[new]
    pub fn new(symbol: String, quantity: f64, order_type: OrderType, price: f64, timestamp: &PyDateTime) -> PyResult<Self> {
        let timestamp = timestamp.call_method0("timestamp")?.extract::<f64>()?;
        Ok(Order {
            symbol,
            quantity,
            order_type,
            price,
            timestamp,
        })
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("Order(symbol={}, quantity={}, order_type={:?}, price={}, timestamp={})",
                   self.symbol, self.quantity, self.order_type, self.price, self.timestamp))
    }

    pub fn set_timestamp(&mut self, timestamp: &PyDateTime) -> PyResult<()> {
        self.timestamp = timestamp.call_method0("timestamp")?.extract::<f64>()?;
        Ok(())
    }

    pub fn get_timestamp<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDateTime> {
        PyDateTime::from_timestamp(py, self.timestamp, None)
    }
}