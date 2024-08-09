from quanta_engine import Order, OrderType, RiskValidator, StrategyManager, StrategyStatus
from datetime import datetime

# Set up risk validator
risk_validator = RiskValidator(max_position_size=1000, max_daily_loss=5000, max_order_value=10000)

# Create and validate an order
order = Order("AAPL", 100, OrderType.Market, 150.0, datetime.utcnow())
is_valid = risk_validator.validate_order(order, current_position=500, daily_pnl=-1000)
print(f"Order is valid: {is_valid}")

order = Order("AAPL", 1, OrderType.Market, 150.0, datetime.utcnow())
is_valid = risk_validator.validate_order(order, current_position=500, daily_pnl=-1000)
print(f"Order is valid: {is_valid}")

# Set up strategy manager
strategy_manager = StrategyManager()

# Add strategies
strategy_manager.add_strategy("strat1", "Moving Average Crossover", "user1")
strategy_manager.add_strategy("strat2", "RSI Reversal", "user2")

# Test both strategies are running
active_strategies = strategy_manager.list_active_strategies()
for strategy in active_strategies:
    print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")

# Update strategy status
strategy_manager.update_strategy_status("strat2", StrategyStatus.Paused)

# List active strategies to check if stopped
active_strategies = strategy_manager.list_active_strategies()
for strategy in active_strategies:
    print(f"Active strategy: {strategy.name} (User: {strategy.user_id})")