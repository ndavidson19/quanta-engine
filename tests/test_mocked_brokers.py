import asyncio
import pytest
from quanta_engine import OrderValidator, ValidationRule, Order, OrderType
from datetime import datetime
from typing import Dict, Any, List
from collections import namedtuple
from abc import ABC, abstractmethod
from quanta_engine import OrderExecutor as RustOrderExecutor, OrderValidator, Order
from ib_insync import IB, MarketOrder, LimitOrder, StopOrder

class BrokerAdapter(ABC):
    @abstractmethod
    async def place_order(self, order: Dict[str, Any]) -> Dict[str, Any]:
        pass

class MockIB:
    def qualifyContracts(self, symbol):
        Contract = namedtuple('Contract', ['symbol'])
        return [Contract(symbol=symbol)]

    async def placeOrder(self, contract, order):
        OrderStatus = namedtuple('OrderStatus', ['status', 'filled', 'remaining', 'avgFillPrice'])
        Order = namedtuple('Order', ['orderId'])
        Trade = namedtuple('Trade', ['order', 'orderStatus'])

        # Simulate order placement
        await asyncio.sleep(0.1)  # Simulate network delay

        order_id = hash(f"{contract.symbol}{order.action}{order.totalQuantity}")
        status = "Filled"
        filled = order.totalQuantity
        remaining = 0
        avg_fill_price = getattr(order, 'lmtPrice', 100.0)  # Use limit price if available, else use a dummy price

        order_status = OrderStatus(status=status, filled=filled, remaining=remaining, avgFillPrice=avg_fill_price)
        order = Order(orderId=order_id)
        return Trade(order=order, orderStatus=order_status)

class MockIBAdapter(BrokerAdapter):
    def __init__(self):
        self.ib_client = MockIB()

    async def place_order(self, order: Dict[str, Any]) -> Dict[str, Any]:
        symbol = order['symbol']
        quantity = order['quantity']
        order_type = order['order_type']

        contract = self.ib_client.qualifyContracts(symbol)[0]

        if order_type == 'Market':
            ib_order = namedtuple('MarketOrder', ['action', 'totalQuantity'])('BUY', quantity)
        elif order_type == 'Limit':
            ib_order = namedtuple('LimitOrder', ['action', 'totalQuantity', 'lmtPrice'])('BUY', quantity, order['price'])
        elif order_type == 'Stop':
            ib_order = namedtuple('StopOrder', ['action', 'totalQuantity', 'auxPrice'])('BUY', quantity, order['price'])
        else:
            raise ValueError(f"Unsupported order type: {order_type}")

        trade = await self.ib_client.placeOrder(contract, ib_order)
        await asyncio.sleep(0.1)  # Simulate processing time

        return {
            'order_id': trade.order.orderId,
            'status': trade.orderStatus.status,
            'filled': trade.orderStatus.filled,
            'remaining': trade.orderStatus.remaining,
            'avg_fill_price': trade.orderStatus.avgFillPrice,
        }
    
class IBAdapter(BrokerAdapter):
    def __init__(self, ib_client: IB):
        self.ib_client = ib_client

    async def place_order(self, order: Dict[str, Any]) -> Dict[str, Any]:
        symbol = order['symbol']
        quantity = order['quantity']
        order_type = order['order_type']

        contract = self.ib_client.qualifyContracts(symbol)[0]

        if order_type == 'Market':
            ib_order = MarketOrder('BUY', quantity)
        elif order_type == 'Limit':
            ib_order = LimitOrder('BUY', quantity, order['price'])
        elif order_type == 'Stop':
            ib_order = StopOrder('BUY', quantity, order['price'])
        else:
            raise ValueError(f"Unsupported order type: {order_type}")

        trade = self.ib_client.placeOrder(contract, ib_order)
        await asyncio.sleep(1)  # Give IB a second to process the order

        return {
            'order_id': trade.order.orderId,
            'status': trade.orderStatus.status,
            'filled': trade.orderStatus.filled,
            'remaining': trade.orderStatus.remaining,
            'avg_fill_price': trade.orderStatus.avgFillPrice,
        }


class OrderExecutor:
    def __init__(self, broker_adapter: BrokerAdapter, validator: OrderValidator):
        self.broker_adapter = broker_adapter
        self.rust_executor = RustOrderExecutor(validator, self._execute_callback)

    async def _execute_callback(self, order_dict):
        return await self.broker_adapter.place_order(order_dict)

    async def execute(self, order: Order):
        loop = asyncio.get_event_loop()
        result = await loop.run_in_executor(None, self.rust_executor.execute, order)
        return await result

    async def execute_multiple(self, orders: List[Order]):
        loop = asyncio.get_event_loop()
        results = await loop.run_in_executor(None, self.rust_executor.execute_multiple, orders)
        return await asyncio.gather(*results)

    def queue_order(self, order: Order):
        self.rust_executor.queue_order(order)

    async def execute_queued(self):
        loop = asyncio.get_event_loop()
        results = await loop.run_in_executor(None, self.rust_executor.execute_queued)
        return await asyncio.gather(*results)

@pytest.mark.asyncio
async def test_order_executor():
    print("Running tests...")

    # Create OrderValidator
    validator = OrderValidator()
    validator.add_symbol("AAPL")
    validator.add_symbol("GOOGL")
    validator.set_rule(OrderType.Market, ValidationRule(min_quantity=1, max_quantity=1000, min_price=0, max_price=float('inf')))
    validator.set_rule(OrderType.Limit, ValidationRule(min_quantity=1, max_quantity=1000, min_price=0.01, max_price=10000))

    # Create MockIBAdapter
    mock_ib_adapter = MockIBAdapter()

    # Create OrderExecutor
    
    executor = OrderExecutor(mock_ib_adapter, validator)

    # Test 1: Execute a single market order with datetime
    print("Testing market order with datetime")
    market_order = Order("AAPL", 100, OrderType.Market, 0, datetime.now())
    result = await executor.execute(market_order)
    assert result['status'] == 'Filled'
    assert result['filled'] == 100
    assert result['remaining'] == 0
    print("Market order with datetime passed successfully!")
    print(f"Result: {result}")
    # Test 2: Execute a single limit order with timestamp
    limit_order = Order("GOOGL", 50, OrderType.Limit, 1500.0, datetime.now().timestamp())
    result = await executor.execute(limit_order)
    assert result['status'] == 'Filled'
    assert result['filled'] == 50
    assert result['remaining'] == 0
    assert result['avg_fill_price'] == 1500.0
    print("Limit order with timestamp passed successfully!")
    print(f"Result: {result}")

    # Test 3: Execute multiple orders
    orders = [
        Order("AAPL", 75, OrderType.Market, 0, datetime.now()),
        Order("GOOGL", 30, OrderType.Limit, 1600.0, datetime.now().timestamp()),
    ]
    results = await executor.execute_multiple(orders)
    assert len(results) == 2
    assert all(result['status'] == 'Filled' for result in results)
    print("Multiple orders passed successfully!")
    print(f"Results: {results}")

    # Test 4: Queue orders and execute them
    executor.queue_order(Order("AAPL", 25, OrderType.Limit, 155.0, datetime.now()))
    executor.queue_order(Order("GOOGL", 40, OrderType.Market, 0, datetime.now().timestamp()))
    queued_results = await executor.execute_queued()
    assert len(queued_results) == 2
    assert all(result['status'] == 'Filled' for result in queued_results)
    print("Queued orders passed successfully!")
    print(f"Results: {queued_results}")

    # Test 5: Try to execute an invalid order (symbol not in validator)
    invalid_order = Order("MSFT", 100, OrderType.Market, 0, datetime.now())
    with pytest.raises(ValueError):
        await executor.execute(invalid_order)
    print("Invalid order test passed successfully!")
    print("Result: ValueError raised as expected")

    # Test 6: Try to execute an order with invalid quantity
    invalid_quantity_order = Order("AAPL", 1001, OrderType.Market, 0, datetime.now())
    with pytest.raises(ValueError):
        await executor.execute(invalid_quantity_order)
    print("Invalid quantity order test passed successfully!")
    print("Result: ValueError raised as expected")

    print("All tests passed successfully!")

if __name__ == "__main__":
    asyncio.run(test_order_executor())