// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `u64` is a 64-bit unsigned integer type,
    // with a range from 0 to 18446744073709551615.
    let value: u64 = 64;
    let min_value: u64 = u64::MIN;
    let max_value: u64 = u64::MAX;

    println!("value: {}", value);   // 64
    println!("min: {}", min_value); // 0
    println!("max: {}", max_value); // 18446744073709551615
}
