use quanta_engine::*;
use pyo3::prelude::*;


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
    assert_eq!(order.symbol, "AAPL");
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
    assert!(validator.symbols.contains(&"AAPL".to_string()));
}

#[test]
fn test_order_validator_set_rule() {
    let mut validator = OrderValidator::new();
    let rule = ValidationRule::new(1.0, 1000.0, 10.0, 500.0);
    validator.set_rule(OrderType::Limit, rule.clone()).unwrap();
    assert_eq!(validator.rules.get(&OrderType::Limit), Some(&rule));
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
        1625140800.0,
        None
    );

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
        let broker_api = py.None();
        manager.add_user("1".to_string(), "Alice".to_string(), broker_api.clone()).unwrap();

        let strategy = py.None();
        assert!(manager.add_strategy("s1".to_string(), "Test Strategy".to_string(), "1".to_string(), strategy).is_ok());
        assert!(manager.strategies.read().unwrap().contains_key("s1"));
    });
}

#[test]
fn test_list_active_strategies() {
    Python::with_gil(|py| {
        let manager = StrategyManager::new();
        let broker_api = py.None();
        manager.add_user("1".to_string(), "Alice".to_string(), broker_api.clone()).unwrap();

        let strategy = py.None();
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
        let broker_api = py.None();
        assert!(manager.add_user("1".to_string(), "Alice".to_string(), broker_api).is_ok());
        assert!(manager.users.read().unwrap().contains_key("1"));
    });
}