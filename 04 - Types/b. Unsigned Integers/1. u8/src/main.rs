// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `u8` is an 8-bit unsigned integer type,
    //  with a range from 0 to 255.
    let value: u8 = 8;
    let min_value: u8 = u8::MIN;
    let max_value: u8 = u8::MAX;

    println!("value: {}", value);   // 8
    println!("min: {}", min_value); // 0
    println!("max: {}", max_value); // 255
}
