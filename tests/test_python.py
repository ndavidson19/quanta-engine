import pytest
from quanta_engine import Order, OrderType, RiskValidator, StrategyManager, StrategyStatus, OrderValidator, ValidationRule, User
from datetime import datetime, timedelta

class Strategy:
    '''
    Mock Strategy implementation
    TODO: move to QuantaForge implementation
    import quantaforge as qf
    class Strategy(qf.Strategy):
        def __init__(self, id, name, user_id, status, user):
            super().__init__(id, name, user_id, status, user)

        def on_tick(self, tick):
            pass
            
        etc...
    '''
    def __init__(self, id, name, user_id, status, user):
        self.id = id
        self.name = name
        self.user_id = user_id
        self.status = status
        self.user = user

from quanta_engine import Order, OrderType, RiskValidator, StrategyManager, StrategyStatus, OrderValidator, ValidationRule, User
from datetime import datetime, timedelta
import logging

logging.basicConfig(level=logging.INFO)

def test_order_validation():
    validator = OrderValidator()
    
    # Add some valid symbols
    validator.add_symbol("AAPL")
    validator.add_symbol("GOOGL")
    
    # Set validation rules
    validator.set_rule(OrderType.Market, ValidationRule(min_quantity=1, max_quantity=1000, min_price=0, max_price=float('inf')))
    validator.set_rule(OrderType.Limit, ValidationRule(min_quantity=1, max_quantity=1000, min_price=0.01, max_price=10000))
    
    # Example 1: Validate a valid order
    valid_order = Order("AAPL", 100, OrderType.Market, 150.0, datetime.now())
    try:
        validator.validate(valid_order)
        print("Valid order is accepted")
    except ValueError as e:
        print(f"Valid order is rejected: {str(e)}")

    # Example 2: Validate an invalid order (wrong symbol)
    invalid_order = Order("INVALID", 100, OrderType.Market, 150.0, datetime.now())
    try:
        validator.validate(invalid_order)
        print("Invalid order is accepted")
    except ValueError as e:
        print(f"Invalid order is rejected: {str(e)}")

    # Example 3: Validate a stale order
    stale_order = Order("AAPL", 100, OrderType.Limit, 150.0, datetime.now() - timedelta(minutes=2))
    try:
        validator.validate(stale_order)
        print("Stale order is accepted")
    except ValueError as e:
        print(f"Stale order is rejected: {str(e)}")

    # Example 4: Batch validation
    orders = [
        Order("AAPL", 100, OrderType.Market, 150.0, datetime.now()),
        Order("GOOGL", 50, OrderType.Limit, 2500.0, datetime.now()),
        Order("INVALID", 75, OrderType.Market, 100.0, datetime.now()),
        Order("AAPL", 200, OrderType.Limit, 140.0, datetime.now() - timedelta(minutes=2)),
    ]

    errors = validator.validate_multiple(orders)
    if errors:
        print("Some orders in the batch are invalid:")
        for index, error in errors:
            print(f"Order at index {index}: {error}")
    else:
        print("All orders in the batch are valid")

def test_risk_validation():
    risk_validator = RiskValidator(max_position_size=1000, max_daily_loss=5000, max_order_value=10000)
    
    # Create and validate orders
    order = Order("AAPL", 100, OrderType.Market, 150.0, datetime.utcnow())
    is_valid = risk_validator.validate_order(order, current_position=500, daily_pnl=-1000)
    print(f"Order is valid: {is_valid}")

    order = Order("AAPL", 1, OrderType.Market, 150.0, datetime.utcnow())
    is_valid = risk_validator.validate_order(order, current_position=500, daily_pnl=-1000)
    print(f"Order is valid: {is_valid}")

def test_strategy_manager():
    strategy_manager = StrategyManager()

    # Create user objects
    user1 = User("user1", "User One", "IKBR")
    user2 = User("user2", "User Two", "Crypto")

    # Add users to the StrategyManager
    strategy_manager.add_user(user1.id, user1.name, user1.get_broker_api())
    strategy_manager.add_user(user2.id, user2.name, user2.get_broker_api())

    # Create strategies
    strategy_obj1 = Strategy("strat1", "Strategy One", "user1", StrategyStatus.Active, user1)
    strategy_obj2 = Strategy("strat2", "Strategy Two", "user2", StrategyStatus.Active, user2)

    strategy_manager.add_strategy("strat1", "Strategy One", "user1", strategy_obj1)
    strategy_manager.add_strategy("strat2", "Strategy Two", "user2", strategy_obj2)

    # Test that both strategies are running
    print("Active Strategies:")
    active_strategies = strategy_manager.list_active_strategies()
    for strategy in active_strategies:
        print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")

    # Update strategy status to Paused for strat2
    strategy_manager.update_strategy_status("strat2", StrategyStatus.Paused)

    # List active strategies to check if strat2 is no longer active
    print("\nActive Strategies after pausing 'strat2':")
    active_strategies = strategy_manager.list_active_strategies()
    for strategy in active_strategies:
        print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")

    # Reactivate strat2 and verify it's active again
    strategy_manager.update_strategy_status("strat2", StrategyStatus.Active)
    print("\nActive Strategies after reactivating 'strat2':")
    active_strategies = strategy_manager.list_active_strategies()
    for strategy in active_strategies:
        print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")

def stress_test_order_validation():
    validator = OrderValidator()
    validator.add_symbol("AAPL")
    validator.set_rule(OrderType.Market, ValidationRule(min_quantity=1, max_quantity=1000, min_price=0, max_price=float('inf')))
    
    # Stress test with a large number of orders
    num_orders = 10000
    orders = [Order("AAPL", i, OrderType.Market, 150.0, datetime.now()) for i in range(num_orders)]
    
    errors = validator.validate_multiple(orders)
    if errors:
        print(f"Some orders in the batch are invalid:")
        for index, error in errors:
            print(f"Order at index {index}: {error}")
    else:
        print(f"All {num_orders} orders in the batch are valid")

if __name__ == "__main__":
    test_order_validation()
    test_risk_validation()
    test_strategy_manager()
    stress_test_order_validation()
