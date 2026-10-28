// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `i8` is a signed 8-bit integer type, with a range from -128 to 127.
    let value: i8 = 8;
    let min_value: i8 = i8::MIN;
    let max_value: i8 = i8::MAX;

    println!("value: {}", value);   // 8
    println!("min: {}", min_value); // -128
    println!("max: {}", max_value); // 127
}
