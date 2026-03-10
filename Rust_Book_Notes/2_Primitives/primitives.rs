/*
Rust provides access to a wide variety of primitives. A sample includes:
    Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    Floating point: f32, f64
    char: Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    bool: either true or false
    The unit type '()' whose only possible value is an empty tuple: ()

*/

use std::mem;

fn main() {
    // A mutable variable's value can be changed.
    // let mut inferred_type = 12; // Type i64 is inferred from another line.
    // inferred_type = 4294967296i64;

    // Array signature consists of Type T and length as [T; length].
    // let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    // let my_tuple = (5u32, 1u8, true, -5.04f32);

    // Integers can be expressed using hexadecimal, octal or binary notation using:
    // these 0x, 0o or 0b

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    // can also do boolean logic
    println!("true AND false is {}", true && false);
    // bitwise operations are allowed as well
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", ys.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
}
