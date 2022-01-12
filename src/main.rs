use std::env;
use std::fs;

pub mod compiler;
use compiler::{parser, tokenizer, typer};
use parser::AST;
use tokenizer::Token;

// use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
// use std::sync::mpsc::channel;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.len() < 2 {
		eprintln!("Usage: reil.exe <source> <target>");
		std::process::exit(1);
	}

	// read_file(&args[0]); // first run
	// compiler(&args[0], &args[1]);
	let source = &args[0];
	let target = &args[1];
	event_router(notify::op::WRITE, source, target);

	std::process::exit(0);

	// // Create a channel to receive the events.
	// let (tx, rx) = channel();

	// // Create a watcher object, delivering raw events.
	// // The notification back-end is selected based on the platform.
	// let mut watcher = raw_watcher(tx).unwrap();

	// // Add a path to be watched. All files and directories at that path and
	// // below will be monitored for changes.
	// watcher.watch(source, RecursiveMode::Recursive).unwrap();

	// loop {
	// 	match rx.recv() {
	// 		Ok(RawEvent {
	// 			path: Some(path),
	// 			op: Ok(op),
	// 			cookie,
	// 		}) => {
	// 			println!("{:?} {:?} ({:?})", op, path, cookie);
	// 			event_router(op, source, target);
	// 		}
	// 		Ok(event) => println!("broken event: {:?}", event),
	// 		Err(e) => println!("watch error: {:?}", e),
	// 	}
	// }
}

fn event_router(operation: notify::Op, source: &String, target: &String) {
	match operation {
		notify::op::WRITE => {
			let input = read_file(source);
			let tokens = tokenizer::tokenizer(&input);
			let token_path = &mut target.clone();
			token_path.push_str(&".tokens".to_string());
			write_file(token_path, &token_string(&tokens));
			//
			//
			let parse_path = &mut target.clone();
			parse_path.push_str(&".ast".to_string());
			match parser::parser(&tokens) {
				Ok(parse) => {
					write_file(parse_path, &ast_string(&parse));

					//
					//
					let typed_parse_path = &mut target.clone();
					typed_parse_path.push_str(&".typed".to_string());
					match typer::typer(&parse) {
						Ok(typed_parse) => {
							write_file(
								typed_parse_path,
								&ast_string(&typed_parse),
							);
						}
						Err(msg) => {
							write_file(typed_parse_path, &msg);
						}
					}
				}
				Err(msg) => {
					write_file(parse_path, &msg);
				}
			}
		}
		_ => {}
	};
}

fn read_file(path: &String) -> String {
	match fs::read_to_string(path) {
		Ok(v) => v,
		Err(e) => {
			eprintln!(
				"Error: failed to read from the file '{}': {:?}",
				path, e
			);
			std::process::exit(1);
		}
	}
}

fn write_file(path: &String, data: &String) {
	match fs::write(path, data) {
		Ok(_v) => {
			// dbg!(v);
		}
		Err(e) => {
			eprintln!("Error: failed to write to file '{}': {:?}", path, e);
			std::process::exit(1);
		}
	};
}

fn token_string(data: &Vec<Token>) -> String {
	let mut output = String::new();
	for group in data {
		output.push_str(
			&format!(
				"{:<12} {:<12} {:<4} {:?}\n",
				format!("{:?}", group.of.kind),
				format!("{:?}", group.of.name),
				group.meta.line,
				group.meta.text,
			)[..],
		)
	}
	output
}

fn ast_string(data: &AST) -> String {
	format!("{:#?}", data)
}
