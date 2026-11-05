// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // Demonstrating overflow handling with `u8`.
    // `u8` is an 8-bit unsigned integer type, with a range from 0 to 255.
    let result = u8::MAX.checked_add(1);

    println!("{result:?}"); // None, because adding 1 to u8::MAX would overflow
}
