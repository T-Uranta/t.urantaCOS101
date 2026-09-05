fn main() {
    // Quantities
    let quantity = [2, 1, 3, 3, 1];

    // Amount per item
    let amount = [
        450_000.00,
        1_500_000.00,
        750_000.00,
        2_850_000.00,
        250_000.00,
    ];

    let mut total_sales = 0.0;

    // Multiply quantity by amount for each item
    for i in 0..quantity.len() {
        total_sales += quantity[i] as f64 * amount[i];
    }

    // Calculate average
    let average = total_sales / quantity.len() as f64;

    println!("Total Sales: ₦{:.2}", total_sales);
    println!("Average Sales: ₦{:.2}", average);
}