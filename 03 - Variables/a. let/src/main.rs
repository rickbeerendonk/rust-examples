// European Union Public License version 1.2
// Copyright © 2024 Rick Beerendonk

fn main() {
    // == Default integer types ==

    // `let` creates a local binding, immutable by default.
    let explicit_type1: i32 = 123;
    let inferred_type1 = 456;

    // Rust infers the second binding's type from its value.
    println!(
        "explicit: {} ({})",
        explicit_type1,
        std::any::type_name_of_val(&explicit_type1)
    );
    println!(
        "inferred: {} ({})",
        inferred_type1,
        std::any::type_name_of_val(&inferred_type1)
    );

    // Reassigning an immutable binding is an error; use `mut` to update it.
    // explicit_type1 = inferred_type1;

    // == Non-default integer types ==

    let explicit_type2: i64 = 123;
    let inferred_type2 = 456_i64;

    // Rust infers the second binding's type from its value.
    println!(
        "explicit: {} ({})",
        explicit_type2,
        std::any::type_name_of_val(&explicit_type2)
    );
    println!(
        "inferred: {} ({})",
        inferred_type2,
        std::any::type_name_of_val(&inferred_type2)
    );


    // Reassigning an immutable binding is an error; use `mut` to update it.
    // explicit_type2 = inferred_type2;
}
