// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `i32` is a 32-bit signed integer type,
    // with a range from -2_147_483_648 to 2_147_483_647.
    let value: i32 = -32;
    let min_value: i32 = i32::MIN;
    let max_value: i32 = i32::MAX;

    println!("value: {}", value);   // -32
    println!("min: {}", min_value); // -2147483648
    println!("max: {}", max_value); // 2147483647
}
