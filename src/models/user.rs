use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct User {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    broker_api: Arc<PyObject>,
}

#[pymethods]
impl User {
    #[new]
    pub fn new(id: String, name: String, broker_api: PyObject) -> Self {
        User {
            id,
            name,
            broker_api: Arc::new(broker_api),
        }
    }

    pub fn get_broker_api(&self) -> PyObject {
        self.broker_api.as_ref().clone()
    }
}