// European Union Public License version 1.2
// Copyright © 2026 Rick Beerendonk

fn main() {
    // `i128` is a 128-bit signed integer type,
    //  with a range from -170_141_183_460_469_231_731_687_303_715_884_105_728 to 170_141_183_460_469_231_731_687_303_715_884_105_727.
    let value: i128 = -128;
    let min_value: i128 = i128::MIN;
    let max_value: i128 = i128::MAX;

    println!("value: {}", value);   // -128
    println!("min: {}", min_value); // -170141183460469231731687303715884105728
    println!("max: {}", max_value); // 170141183460469231731687303715884105727
}
