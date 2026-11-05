// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `f64` is a 64-bit floating point type, used for decimal numbers.
    // It can represent numbers with fractional parts, with higher precision than `f32`.
    let value: f64 = 64.5;
    let min_value: f64 = f64::MIN;
    let max_value: f64 = f64::MAX;

    println!("value: {}", value);       // 64.5
    println!("min: {}", min_value);     // -1.7976931348623157e308
    println!("max: {}", max_value);     // 1.7976931348623157e308
}
