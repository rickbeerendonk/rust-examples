// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `isize` is a pointer-sized signed integer type,
    // with a range that depends on the architecture (32-bit or 64-bit).
    let value: isize = -1;
    let min_value: isize = isize::MIN;
    let max_value: isize = isize::MAX;

    println!("value: {}", value);   // -1
    println!("min: {}", min_value); // depends on architecture
    println!("max: {}", max_value); // depends on architecture
}
