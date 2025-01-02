// European Union Public License version 1.2
// Copyright © 2024 Rick Beerendonk

fn main() {
    // Explicit type
    let explicit: i32 = 123;

    // Implicit type
    let implicit = 456;

    // To print the type of the variables
    println!("{}", std::any::type_name_of_val(&explicit)); // i32
    println!("{}", std::any::type_name_of_val(&implicit)); // i32

    println!("{}", explicit);
    println!("{}", implicit);

    // error: let cannot be reassigned
    // explicit = implicit;
}