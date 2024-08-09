use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashMap;
use pyo3::types::PyDateTime;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "Market"),
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Stop => write!(f, "Stop"),
            OrderType::StopLimit => write!(f, "StopLimit"),
        }
    }
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
    #[pyo3(get, set)]
    pub timestamp: f64,
    #[pyo3(get)]
    pub additional_data: Option<HashMap<String, String>>,
}

#[pymethods]
impl Order {
    #[new]
    pub fn new(symbol: String, quantity: f64, order_type: OrderType, price: f64, timestamp: &PyAny) -> PyResult<Self> {
        let timestamp = if timestamp.is_instance_of::<PyDateTime>() {
            timestamp.call_method0("timestamp")?.extract::<f64>()?
        } else {
            timestamp.extract::<f64>()?
        };
        
        Ok(Order {
            symbol,
            quantity,
            order_type,
            price,
            timestamp,
            additional_data: None,
        })
    }

    pub fn set_additional_data(&mut self, data: HashMap<String, String>) {
        self.additional_data = Some(data);
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("Order(symbol={}, quantity={}, order_type={:?}, price={}, timestamp={})",
                   self.symbol, self.quantity, self.order_type, self.price, self.timestamp))
    }

    pub fn set_timestamp_py(&mut self, timestamp: &PyDateTime) -> PyResult<()> {
        self.timestamp = timestamp.call_method0("timestamp")?.extract::<f64>()?;
        Ok(())
    }

    pub fn get_timestamp_py<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDateTime> {
        PyDateTime::from_timestamp(py, self.timestamp, None)
    }
}