fn main() {
	// Line-Comment
	/*
	* Block Comment
	* /// Three slashes are parsed into HTML documentation
	*/

	/* You can put /* inside itself */
	and it just will repeat them /*  /* 
	*/  */ /* */ cdscds*/

	let x = "!";
	let y = "Hello";
	// In general, the `{}` will be automatically replaced with any arguments but it is also reccomended to number your prints as such
	println!("{0} World{1}
	{1}\n{2}", y, x, (23 - 6) % 5 + 20 * 30 / (3 + 4));
	// you can also do named arguments like this: 
	// As can named arguments.
	println!("{subject} {verb} {object}",
	object="the lazy dog",
	subject="the quick brown fox",
	verb="jumps over");

	// Different formatting can be invoked by specifying the format character
	// after a `:`.
	println!("Base 10:               {}",   69420); // 69420
	println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
	println!("Base 8 (octal):        {:o}", 69420); // 207454
	println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

	// You can right-justify text with a specified width. This will
	println!("{number:>5}", number=1); // output "    1" (Four white spaces and a "1", for a total width of 5.)
	println!("{number:0>5}", number=1); // 00001
	println!("{number:0<5}", number=1); // 10000
	println!("{number:0>width$}", number=1, width=5);  // you need to append the $ to specify width

	// All types which want to use std::fmt formatting traits require an implementation to be printable.
	// if you dont have this, you cant print it, so you cant print a struct by default but there are ways around this
	// Derive the `fmt::Debug` implementation for `Structure` since struct by itself is not part of ftm::
	#[derive(Debug)]
	#[allow(dead_code)] // Suppress the warning for this struct since it is only used to println
	struct Person<'a> {
		name: &'a str,
		age: u8
	}

	let name = "Peter";
	let age = 27;
	let peter = Person { name, age };

	// If you want to print a struct, you can probably find a better way to do this
	println!("{:#?}", peter);

	// if you have a function like write!(f,"{}",value); you can add a ? before the ; to check if it errors out
	// if it errors out, the error is returned and the code is returned. 

	// Another useful thing is format: https://doc.rust-lang.org/std/fmt/#formatting-traits
	// format is like printf in C, and here is how you would use it: let result = format!("This is a string with {} and {}", value1, value2);
	// execpt now result would be equal to that string
}
