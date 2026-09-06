pub mod template;

// Use this file to add helper functions and additional modules.

pub fn day10_expand(input: &str) -> String {
    // moved here to use criterion
    // original attempt had lots of alloc and was sloooow... Vec was faster, but plain string is even faster...

    let cap = input.len() as f32 * 1.303577; // will grow on average by Conway's constant
    // let mut arr = Vec::with_capacity(cap as usize);

    let mut result = String::with_capacity(cap as usize);

    let mut prev_c = input.chars().nth(0).unwrap();
    let mut count = 1;
    for c in input.chars().skip(1) {
        if c == prev_c {
            count += 1;
        } else {
            result.push(char::from_digit(count, 10).unwrap());
            result.push(prev_c);
            count = 1;
            prev_c = c;
        }
    }
    result.push(char::from_digit(count, 10).unwrap());
    result.push(prev_c);
    result
}
