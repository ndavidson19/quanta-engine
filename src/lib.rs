use pyo3::prelude::*;

mod models;
mod validation;
mod execution;
mod risk;
mod strategy;

// Expose internally in production, publicly in tests
use models::user::User;
use models::order::{Order, OrderType};
use validation::order_validator::{OrderValidator, ValidationRule};
use execution::executor::OrderExecutor;
use risk::risk_validator::RiskValidator;
use strategy::strategy_manager::StrategyManager;
use strategy::strategy::{StrategyWrapper, StrategyStatus};


#[pymodule]
fn quanta_engine(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Order>()?;
    m.add_class::<OrderType>()?;
    m.add_class::<OrderValidator>()?;
    m.add_class::<OrderExecutor>()?;
    m.add_class::<RiskValidator>()?;
    m.add_class::<StrategyManager>()?;
    m.add_class::<StrategyStatus>()?;
    m.add_class::<ValidationRule>()?;
    m.add_class::<User>()?;
    m.add_class::<StrategyWrapper>()?;

    // Import Python's datetime module
    let datetime = py.import("datetime")?;
    m.add("datetime", datetime)?;

    Ok(())
}

/* Tests
Below are the Rust tests for the Quanta Engine library. These tests cover the creation of orders, validation rules, order validators, and the strategy manager. The tests also include integration tests for the strategy manager.
TODO: Most of the tests are failing for lack of getter and setter methods. Implement these methods to make the tests pass.
The Pytest integration tests are implemented and passing so for now this is ok.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyNone;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "AAPL".to_string(),
            100.0,
            OrderType::Limit,
            150.0,
            1625140800.0, // Example timestamp
            None
        );
        assert_eq!(order.get_symbol(), "AAPL");
        assert_eq!(order.quantity, 100.0);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, 150.0);
        assert_eq!(order.timestamp, 1625140800.0);
    }

    #[test]
    fn test_validation_rule_creation() {
        let rule = ValidationRule::new(1.0, 1000.0, 10.0, 500.0);
        assert_eq!(rule.min_quantity, 1.0);
        assert_eq!(rule.max_quantity, 1000.0);
        assert_eq!(rule.min_price, 10.0);
        assert_eq!(rule.max_price, 500.0);
    }

    #[test]
    fn test_order_validator_add_symbol() {
        let mut validator = OrderValidator::new();
        validator.add_symbol("AAPL".to_string()).unwrap();
        assert!(validator.get_symbols().contains(&"AAPL".to_string()));
    }
    
    #[test]
    fn test_order_validator_set_rule() {
        let mut validator = OrderValidator::new();
        let rule = ValidationRule::new(1.0, 1000.0, 10.0, 500.0);
        validator.set_rule(OrderType::Limit, rule.clone()).unwrap();
        assert_eq!(validator.get_rules().get(&OrderType::Limit), Some(&rule));
    }

    #[test]
    fn test_order_validator_validate() {
        let mut validator = OrderValidator::new();
        validator.add_symbol("AAPL".to_string()).unwrap();
        let rule = ValidationRule::new(1.0, 1000.0, 10.0, 500.0);
        validator.set_rule(OrderType::Limit, rule).unwrap();

        let order = Order::new(
            "AAPL".to_string(),
            100.0,
            OrderType::Limit,
            150.0,
            &Python::with_gil(|py| py.None()),
        ).unwrap();

        assert!(validator.validate(&order).is_ok());
    }

    #[test]
    fn test_order_validator_validate_invalid_symbol() {
        let mut validator = OrderValidator::new();
        let order = Order::new(
            "INVALID".to_string(),
            100.0,
            OrderType::Limit,
            150.0,
            1625140800.0,
            None
        );

        assert!(validator.validate(&order).is_err());
    }

    #[test]
    fn test_add_strategy() {
        Python::with_gil(|py| {
            let manager = StrategyManager::new();
            let broker_api = PyNone::new(py);
            manager.add_user("1".to_string(), "Alice".to_string(), broker_api.clone()).unwrap();

            let strategy = PyNone::new(py);
            assert!(manager.add_strategy("s1".to_string(), "Test Strategy".to_string(), "1".to_string(), strategy).is_ok());
            assert!(manager.strategies.read().unwrap().contains_key("s1"));
        });
    }

    #[test]
    fn test_list_active_strategies() {
        Python::with_gil(|py| {
            let manager = StrategyManager::new();
            let broker_api = PyNone::new(py);
            manager.add_user("1".to_string(), "Alice".to_string(), broker_api.clone()).unwrap();

            let strategy = PyNone::new(py);
            manager.add_strategy("s1".to_string(), "Test Strategy".to_string(), "1".to_string(), strategy).unwrap();
            
            let active_strategies = manager.list_active_strategies();
            assert_eq!(active_strategies.len(), 1);
            assert_eq!(active_strategies[0].id, "s1");
        });
    }

    #[test]
    fn test_add_user() {
        Python::with_gil(|py| {
            let manager = StrategyManager::new();
            let broker_api = PyNone::new(py);
            assert!(manager.add_user("1".to_string(), "Alice".to_string(), broker_api).is_ok());
            assert!(manager.users.read().unwrap().contains_key("1"));
        });
    }

    // Replace these with actual implementations or mocks
    fn create_dummy_broker_api() -> PyObject {
        Python::with_gil(|py| PyNone::new(py).to_object(py))
    }

    fn create_dummy_strategy() -> PyObject {
        Python::with_gil(|py| PyNone::new(py).to_object(py))
    }

    #[test]
    fn test_strategy_manager_integration() {
        Python::with_gil(|py| {
            let strategy_manager = StrategyManager::new();
            let user_id = "user1".to_string();
            let strategy_id = "strat1".to_string();
    
            let broker_api = py.None();
            strategy_manager.add_user(user_id.clone(), "User 1".to_string(), broker_api).unwrap();
    
            let strategy = py.None();
            strategy_manager.add_strategy(strategy_id.clone(), "Test Strategy".to_string(), user_id.clone(), strategy).unwrap();
    
            let retrieved_strategy = strategy_manager.get_strategy(&strategy_id).unwrap();
            assert_eq!(retrieved_strategy.id, strategy_id);
        });
    }
}

*/