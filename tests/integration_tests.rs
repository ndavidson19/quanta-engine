use quanta_engine::*;

#[test]
fn test_strategy_manager_integration() {
    let strategy_manager = StrategyManager::new();
    let user_id = "user1".to_string();
    let strategy_id = "strat1".to_string();

    // Assuming you have a function to create a dummy Python object for the broker API
    let broker_api = create_dummy_broker_api();

    strategy_manager.add_user(user_id.clone(), "User 1".to_string(), broker_api).unwrap();

    let strategy = create_dummy_strategy(); // Replace with actual strategy creation

    strategy_manager.add_strategy(strategy_id.clone(), "Test Strategy".to_string(), user_id.clone(), strategy).unwrap();

    let retrieved_strategy = strategy_manager.get_strategy(&strategy_id).unwrap();
    assert_eq!(retrieved_strategy.id, strategy_id);
}