// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `u128` is a 128-bit unsigned integer type,
    // with a range from 0 to 340282366920938463463374607431768211455.
    let value: u128 = 128;
    let min_value: u128 = u128::MIN;
    let max_value: u128 = u128::MAX;

    println!("value: {}", value);   // 128
    println!("min: {}", min_value); // 0
    println!("max: {}", max_value); // 340282366920938463463374607431768211455
}
