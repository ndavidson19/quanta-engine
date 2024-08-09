use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::VecDeque;
use crate::models::order::{Order, OrderType};
use crate::validation::order_validator::OrderValidator;

#[pyclass]
pub struct OrderExecutor {
    validator: OrderValidator,
    #[pyo3(get)]
    python_callback: PyObject,
    order_queue: VecDeque<Order>,
}

#[pymethods]
impl OrderExecutor {
    #[new]
    fn new(validator: OrderValidator, python_callback: PyObject) -> Self {
        OrderExecutor {
            validator,
            python_callback,
            order_queue: VecDeque::new(),
        }
    }

    fn prepare_order(&self, py: Python, order: &Order) -> PyResult<PyObject> {
        // Validate the order
        self.validator.validate(order)?;

        // Prepare order data for Python
        let order_dict = PyDict::new(py);
        order_dict.set_item("symbol", &order.symbol)?;
        order_dict.set_item("quantity", order.quantity)?;
        order_dict.set_item("order_type", order.order_type.to_string())?;
        
        // Add optional fields based on order type
        match order.order_type {
            OrderType::Market => {},
            OrderType::Limit | OrderType::Stop | OrderType::StopLimit => {
                order_dict.set_item("price", order.price)?;
            },
        }

        // Add any additional fields
        if let Some(additional_data) = &order.additional_data {
            for (key, value) in additional_data {
                order_dict.set_item(key, value)?;
            }
        }

        Ok(order_dict.into())
    }

    fn queue_order(&mut self, order: Order) -> PyResult<()> {
        self.order_queue.push_back(order);
        Ok(())
    }

    fn execute_queued(&mut self, py: Python) -> PyResult<PyObject> {
        let mut results = Vec::new();
        while let Some(order) = self.order_queue.pop_front() {
            let prepared_order = self.prepare_order(py, &order)?;
            let result = self.python_callback.call1(py, (prepared_order,))?;
            results.push(result);
        }
        Ok(PyList::new(py, results).into())
    }

    fn execute(&self, py: Python, order: &Order) -> PyResult<PyObject> {
        let prepared_order = self.prepare_order(py, order)?;
        self.python_callback.call1(py, (prepared_order,))
    }

    fn execute_multiple(&self, py: Python, orders: &PyList) -> PyResult<PyObject> {
        let mut results = Vec::new();
        for order in orders.iter() {
            let order: Order = order.extract()?;
            let result = self.execute(py, &order)?;
            results.push(result);
        }
        Ok(PyList::new(py, results).into())
    }
}