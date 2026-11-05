// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `i64` is a 64-bit signed integer type,
    // with a range from -9_223_372_036_854_775_808 to 9_223_372_036_854_775_807.
    let value: i64 = -64;
    let min_value: i64 = i64::MIN;
    let max_value: i64 = i64::MAX;

    println!("value: {}", value);   // -64
    println!("min: {}", min_value); // -9223372036854775808
    println!("max: {}", max_value); // 9223372036854775807
}
