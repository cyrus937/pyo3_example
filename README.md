# pyo3_example

## Start the project

Once you've installed rust, you'll need to follow these steps to launch the project:

1. Go to the project folder
```
cd project
```
2. Create and active a python virtual environment
```
python -m venv .env
source .env/bin/activate
```
2. Install maturin for building and publishing Rust-based Python packages with minimal configuration
```
pip install maturin
```
3. Build and execute the module
```
maturin develop

python py_scripts/test.py
```