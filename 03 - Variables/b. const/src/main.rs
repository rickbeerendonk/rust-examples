// European Union Public License version 1.2
// Copyright © 2024 Rick Beerendonk

fn main() {
    // Explicit type
    const EXPLICIT: i32 = 123;

    // Implicit type not possible
    //const IMPLICIT = 456;

    println!("{}", EXPLICIT);

    // error: let cannot be reassigned
    //EXPLICIT = 456;
}