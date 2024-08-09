use pyo3::prelude::*;
use crate::models::order::Order;

#[pyclass]
pub struct RiskValidator {
    max_position_size: f64,
    max_daily_loss: f64,
    max_order_value: f64,
}

#[pymethods]
impl RiskValidator {
    #[new]
    fn new(max_position_size: f64, max_daily_loss: f64, max_order_value: f64) -> Self {
        RiskValidator {
            max_position_size,
            max_daily_loss,
            max_order_value,
        }
    }

    fn validate_order(&self, order: &Order, current_position: f64, daily_pnl: f64) -> PyResult<bool> {
        if order.quantity.abs() > self.max_position_size {
            return Ok(false);
        }

        if order.quantity * order.price > self.max_order_value {
            return Ok(false);
        }

        if daily_pnl < -self.max_daily_loss {
            return Ok(false);
        }

        if (current_position + order.quantity).abs() > self.max_position_size {
            return Ok(false);
        }

        Ok(true)
    }
}