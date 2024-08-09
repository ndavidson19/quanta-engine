import unittest
from trade_engine import TradeEngine

class TestTradeEngine(unittest.TestCase):
    def test_execute_order(self):
        engine = TradeEngine()
        result = engine.execute_order("AAPL", 100)
        self.assertIn("Executed order: 100.0 shares of AAPL", result)

    def test_get_position(self):
        engine = TradeEngine()
        engine.execute_order("AAPL", 100)
        self.assertEqual(engine.get_position("AAPL"), 100.0)

if __name__ == '__main__':
    unittest.main()