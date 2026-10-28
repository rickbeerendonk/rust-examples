// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `i16` is a signed 16-bit integer type, with a range from -32.768 to 32.767
    let value: i16 = 16;
    let min_value: i16 = i16::MIN;
    let max_value: i16 = i16::MAX;

    println!("value: {}", value);   // 16
    println!("min: {}", min_value); // -32768
    println!("max: {}", max_value); // 32767
}
