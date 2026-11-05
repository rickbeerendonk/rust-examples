// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `f32` is a 32-bit floating point type, used for decimal numbers.
    // It can represent numbers with fractional parts, but with limited precision.
    let value: f32 = 32.5;
    let min_value: f32 = f32::MIN;
    let max_value: f32 = f32::MAX;

    println!("value: {}", value);   // 32.5
    println!("min: {}", min_value); // -3.4028235e38
    println!("max: {}", max_value); // 3.4028235e38
}
