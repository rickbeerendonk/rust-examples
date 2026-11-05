// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `u32` is a 32-bit unsigned integer type,
    // with a range from 0 to 4294967295.
    let value: u32 = 32;
    let min_value: u32 = u32::MIN;
    let max_value: u32 = u32::MAX;

    println!("value: {}", value);   // 32
    println!("min: {}", min_value); // 0
    println!("max: {}", max_value); // 4294967295
}
