// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `usize` is an unsigned integer type that is typically used for indexing and memory-related operations.
    // Its size depends on the architecture (e.g., 32-bit on a 32-bit system, 64-bit on a 64-bit system).
    let item_count: usize = 1;
    let min_value: usize = usize::MIN;
    let max_value: usize = usize::MAX;

    println!("item count: {}", item_count); // 1
    println!("min: {}", min_value);         // 0
    println!("max: {}", max_value);         // depends on architecture
}
