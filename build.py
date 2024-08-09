import subprocess
import sys

def build_rust_extension():
    try:
        # Build the Rust extension
        subprocess.check_call([sys.executable, '-m', 'maturin', 'develop'])
        print("Rust extension built successfully")
        
        # Install the package in editable mode
        subprocess.check_call([sys.executable, '-m', 'pip', 'install', '-e', '.'])
        print("Package installed in editable mode")
    except subprocess.CalledProcessError as e:
        print(f"Error building Rust extension: {e}")
        sys.exit(1)

if __name__ == "__main__":
    build_rust_extension()