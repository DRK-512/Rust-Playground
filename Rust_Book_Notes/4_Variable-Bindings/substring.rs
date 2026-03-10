/*
 * Write a function that takes a string of words separated by spaces and
 * returns the first word it finds in that string. If the function doesn’t
 * find a space in the string, the whole string must be one word, so the
 * entire string should be returned
 *
 */

fn sub_string(str: String) {
    for i in str.chars() {
        if i == ' ' {
            return;
        }
        print!("{0}", i);
    }
}

fn main() {
    let my_str = String::from("Hello World!");
    // We want this var to go out of scope after the function call
    sub_string(my_str);
}
