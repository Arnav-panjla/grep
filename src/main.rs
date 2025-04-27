use std::{env, fs, process};
use minigrep::*;
fn main() {
	let args: Vec<String> = env::args().collect();

	let config = Config::build(&args).unwrap_or_else(|err|{
	eprintln!("Problem parsing argumets {err}");
	process::exit(1);
	});
	
	let _ = run(config);
}

