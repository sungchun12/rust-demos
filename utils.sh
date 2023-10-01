# setup virtual environment
mkdir string_sum
cd string_sum
python -m venv .env
source .env/bin/activate
pip install maturin

# setup rust extention project structure
maturin init --bindings pyo3

# build rust extention
maturin develop

# try out the package in python
python string_sum_demo.py