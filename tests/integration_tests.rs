use quanta_engine::{StrategyManager, Strategy, StrategyStatus};

#[test]
fn test_strategy_manager() {
    let mut manager = StrategyManager::new();
    
    manager.add_strategy("strat1".to_string(), "Strategy 1".to_string(), "user1".to_string()).unwrap();
    manager.add_strategy("strat2".to_string(), "Strategy 2".to_string(), "user2".to_string()).unwrap();
    
    assert_eq!(manager.list_active_strategies().len(), 2);
    
    manager.update_strategy_status("strat1", StrategyStatus::Paused).unwrap();
    
    assert_eq!(manager.list_active_strategies().len(), 1);
    
    let strategy = manager.get_strategy("strat2").unwrap();
    assert_eq!(strategy.name, "Strategy 2");
}