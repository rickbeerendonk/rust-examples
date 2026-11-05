// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `u16` is a 16-bit unsigned integer type,
    // with a range from 0 to 65535.
    let value: u16 = 16;
    let min_value: u16 = u16::MIN;
    let max_value: u16 = u16::MAX;

    println!("value: {}", value);  // 16
    println!("min: {}", min_value); // 0
    println!("max: {}", max_value); // 65535
}
