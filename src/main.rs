use polars::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let df_customers = df! (
        "customer_id" => &[1, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    )?;

    println!("{}", &df_customers);
    
    Ok(())
}
