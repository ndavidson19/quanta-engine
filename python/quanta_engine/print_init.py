import os

init_path = os.path.join(os.path.dirname(__file__), '__init__.py')

print("Contents of __init__.py:")
try:
    with open(init_path, 'r') as f:
        print(f.read())
except FileNotFoundError:
    print("__init__.py file not found!")
except Exception as e:
    print(f"Error reading __init__.py: {e}")