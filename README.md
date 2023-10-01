# rust-demos

Build a python extension to run rust code in python.

```shell
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

# output example
'25'
```

# Build an executable file with rust

To run Rust code, you have to install Rust on your system first. Please follow the instructions in the official Rust documentation to install it.

After installing Rust, you need to create a new Rust project.

1.	Open the terminal and create a new project using cargo:

```shell
cargo new my_project
```

Replace ﻿my_project with your desired project name.

2.	Navigate to the new project directory:

```shell
cd my_project
```

3.	Replace the contents of the ﻿src/main.rs file with your code. Your code should use the ﻿polars crate, so you'll need to import it into your project. This can be done by adding it to your ﻿Cargo.toml file:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2018"

[dependencies]
polars = "0.15.1"
```

Ensure to update the `polars` version in the dependencies to the most recent version available.

4.	The code should then look like this:

```rust
use polars::prelude::*;

fn main() -> polars::error::Result<()> {
    let df_customers = df! (
        "customer_id" => &[1, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    )?;

    println!("{}", &df_customers);

    Ok(())
}
```

5.	Run the code from the terminal:

```shell
cargo run

# example output
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rust-demos`
shape: (3, 2)
┌─────────────┬─────────┐
│ customer_id ┆ name    │
│ ---         ┆ ---     │
│ i32         ┆ str     │
╞═════════════╪═════════╡
│ 1           ┆ Alice   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 2           ┆ Bob     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 3           ┆ Charlie │
└─────────────┴─────────┘
```

This will install polars and any other dependencies, then compile and run your program.

6. Build and run the executable file

```shell
# build executable file
cargo build --release

# run executable file
./target/release/rust-demos 

# example output
shape: (3, 2)
┌─────────────┬─────────┐
│ customer_id ┆ name    │
│ ---         ┆ ---     │
│ i32         ┆ str     │
╞═════════════╪═════════╡
│ 1           ┆ Alice   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 2           ┆ Bob     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 3           ┆ Charlie │
└─────────────┴─────────┘
```
