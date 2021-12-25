use std::env;
use std::fs;

fn main() {
	let filepath = "./example/test.reil";
	println!("In file {}", filepath);

	let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");

	println!("With text:\n{}", contents);
}
