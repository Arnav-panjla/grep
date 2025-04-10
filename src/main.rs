use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	let query = &args[1];
	let path = &args[2];
	println!("searching for '{query}' in {path}");
	let content = fs::read_to_string(path)
	.expect("pls enter a valid file path !");
	println!("{content}");
}
