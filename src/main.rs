// use std::env;
// use std::fs;

// fn main() {
// 	let filepath = "./example/test.reil";
// 	println!("In file {}", filepath);

// 	let contents = fs::read_to_string(filepath)
// 		.expect("Something went wrong reading the file");

// 	println!("With text:\n{}", contents);
// }

extern crate notify;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch() -> notify::Result<()> {
	// Create a channel to receive the events.
	let (tx, rx) = channel();

	// Automatically select the best implementation for your platform.
	// You can also access each implementation directly e.g. INotifyWatcher.
	let mut watcher: RecommendedWatcher =
		Watcher::new(tx, Duration::from_secs(2))?;

	// Add a path to be watched. All files and directories at that path and
	// below will be monitored for changes.
	watcher.watch("./example/test.reil", RecursiveMode::Recursive)?;

	// This is a simple loop, but you may want to use more complex logic here,
	// for example to handle I/O.
	loop {
		match rx.recv() {
			Ok(event) => println!("{:?}", event),
			Err(e) => println!("watch error: {:?}", e),
		}
	}
}

fn main() {
	if let Err(e) = watch() {
		println!("error: {:?}", e)
	}
}
