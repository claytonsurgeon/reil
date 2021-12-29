use std::env;
use std::fs;
use std::path;

// fn main() {
// 	let filepath = "./example/test.reil";
// 	println!("In file {}", filepath);

// 	let contents = fs::read_to_string(filepath)
// 		.expect("Something went wrong reading the file");

// 	println!("With text:\n{}", contents);
// }

// extern crate notify;

use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.len() < 1 {
		eprintln!("Usage: reil.exe <source>");
		std::process::exit(1);
	}

	read_file(&args[0]); // first run

	// Create a channel to receive the events.
	let (tx, rx) = channel();

	// Create a watcher object, delivering raw events.
	// The notification back-end is selected based on the platform.
	let mut watcher = raw_watcher(tx).unwrap();

	// Add a path to be watched. All files and directories at that path and
	// below will be monitored for changes.
	watcher.watch(&args[0], RecursiveMode::Recursive).unwrap();

	loop {
		match rx.recv() {
			Ok(RawEvent {
				path: Some(path),
				op: Ok(op),
				cookie,
			}) => {
				println!("{:?} {:?} ({:?})", op, path, cookie);
				event_router(
					op,
					&path.into_os_string().into_string().unwrap(),
				);
			}
			Ok(event) => println!("broken event: {:?}", event),
			Err(e) => println!("watch error: {:?}", e),
		}
	}
}

fn event_router(operation: notify::Op, path: &String) {
	match operation {
		notify::op::WRITE => {
			read_file(path);
		}
		_ => {}
	};
}

fn read_file(path: &String) {
	let entryfile = match fs::read_to_string(path) {
		Ok(v) => v,
		Err(e) => {
			eprintln!(
				"Error: failed to read from the file '{}': {:?}",
				path, e
			);
			std::process::exit(1);
		}
	};

	// dbg!(entryfile);
	tokenizer(&entryfile);
	// dbg!(entryfile);
	// unimplemented!();
}

fn tokenizer(input: &String) {
	println!("{}", input);
}
