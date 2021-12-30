use lazy_static::lazy_static;
use regex::Regex;
// use std::cmp::Ordering;
// use std::fmt::Display;
// use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum Category {
	Skip,
	Newline,
	Comma,
	Semicolon,
	Arrow,
	Number,
	ReservedWord,
	Operator,
	Paren,
	Squaren,
	Bracket,
	Colon,
	String,
	// Key,
	Word,
	Invalid,
}

pub fn tokenizer(input: &String) -> Vec<(Category, String)> {
	// let mut spec: Vec<&Regex> = Vec::new();
	lazy_static! {
		static ref SPEC: Vec<(Category, Regex)> =
			vec![
				// Whitespace
				// (Category::Newline, Regex::new(r"^(\s|\t)*(\n|\r)+(\s|\t)*").unwrap()),
				(Category::Newline, Regex::new(r"^(\n|\r)+").unwrap()),
				(Category::Skip, Regex::new(r"^[[:blank:]]+").unwrap()),

				// Comments
				(Category::Skip, Regex::new(r"^//.*").unwrap()),
				(Category::Skip, Regex::new(r"^/\*[\s\S]*?\*/").unwrap()),

				// Punctuation
				(Category::Comma, Regex::new(r"^[,]").unwrap()),
				(Category::Semicolon, Regex::new(r"^[;]").unwrap()),

				// Function Arrow
				(Category::Arrow, Regex::new(r"^→").unwrap()),
				(Category::Arrow, Regex::new(r"^->").unwrap()),
				// Numbers
				(Category::Number, Regex::new(r"^\-?[0-9]+\.[0-9]*").unwrap()),
				(Category::Number, Regex::new(r"^\-?[0-9]*\.[0-9]+").unwrap()),
				(Category::Number, Regex::new(r"^\-?[0-9]+").unwrap()),

				// Reserved Words
				(Category::ReservedWord, Regex::new(r"^if\b").unwrap()),
				(Category::ReservedWord, Regex::new(r"^else\b").unwrap()),
				(Category::ReservedWord, Regex::new(r"^true\b").unwrap()),
				(Category::ReservedWord, Regex::new(r"^false\b").unwrap()),

				// Operators
				(Category::Operator, Regex::new(r"^[.>~<!*=/%÷×·^'∘+-]+").unwrap()),
				// parens
				(Category::Paren, Regex::new(r"^\(").unwrap()),
				(Category::Paren, Regex::new(r"^\)").unwrap()),

				(Category::Squaren, Regex::new(r"^\[").unwrap()),
				(Category::Squaren, Regex::new(r"^\]").unwrap()),

				(Category::Bracket, Regex::new(r"^\{").unwrap()),
				(Category::Bracket, Regex::new(r"^\}").unwrap()),


				(Category::Colon, Regex::new(r"^:").unwrap()),


				(Category::String, Regex::new(r#"^"[^"]*("|$)"#).unwrap()),


				// (Category::Key, Regex::new(r"^[A-Za-z][A-Za-z0-9]*(?:(\s)*[{])").unwrap()),
				// (Category::Key, Regex::new(r"^[A-Za-z][A-Za-z0-9]*(?:(\s)*[:])").unwrap()),


				(Category::Word, Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap()),

				(Category::Invalid, Regex::new(r"^.").unwrap()),
			];
	}

	dbg!(&SPEC[0]);

	let mut tokens: Vec<(Category, String)> = Vec::new();
	let mut cursor = 0;
	// let mut line = 0;
	let length = input.len();

	'outer: while cursor < length {
		for (category, re) in &SPEC[..] {
			match re.find(&input[cursor..]) {
				Some(mat) => {
					let token_text = &input[cursor..cursor + mat.end()];

					match category {
						// Category::Key => {
						// 	let m = SPEC[26].1.find(token_text).unwrap().end();
						// 	tokens.push((*category, token_text[0..m].to_string()))
						// }
						Category::Skip => {}
						_ => tokens.push((*category, token_text.to_string())),
					}

					cursor += mat.end();
					continue 'outer;
				}
				None => {}
			}
		}
	}

	tokens
}

// let re = Regex::new(r"^dec").unwrap();
// let text = "decode".to_string();
// let bob = &text[..3];

// // println!("Found match? {}", re.is_match(text));

// match re.find(&text) {
// 	Some(mat) => {
// 		// println!("Found match: {}", caps.get(0).unwrap().as_str())
// 		dbg!(&mat);
// 		let jef = &text[mat.end()..];
// 		dbg!(&jef);
// 		// dbg!(&text[mat.end as u32])
// 	}
// 	None => {
// 		println!("Could not find match");
// 	}
// }

// pub fn tokenizer(input: &String) {
// 	let mut spec: Vec<&Regex> = Vec::new();
// 	lazy_static! {
// 		static ref NEWLINE: Regex = Regex::new(r"^\n").unwrap();
// 		static ref WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
// 		static ref COMMENTS: Regex =
// 			Regex::new(r"(^//.*)|(^/\*[\s\S]*?\*/)").unwrap();
// 		static ref NUMBER: Regex = Regex::new(
// 			r"(^\-?[0-9]+\.[0-9]*)|(^\-?[0-9]*\.[0-9]+)|(^\-?[0-9]+)"
// 		)
// 		.unwrap();
// 		static ref PUNCT: Regex = Regex::new(r"^[,;:]").unwrap();
// 		static ref RETURN: Regex = Regex::new(r"(^→)|(^->)").unwrap();
// 		static ref OPERATOR: Regex =
// 			Regex::new(r"^[.>~<!*=/%÷×·^'∘+-]+").unwrap();
// 		static ref PAREN: Regex =
// 			Regex::new(r"(^\()|(^\))|(^\[)|(^\])|(^\{)|(^\})").unwrap();
// 		static ref STRING: Regex =
// 			Regex::new(r#"(^"[^"]*("|$))|(^`[^`]*(`|$))"#).unwrap();
// 		static ref WORD: Regex =
// 			Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap();
// 	}
// 	spec.push(&NEWLINE);
// 	spec.push(&WHITESPACE);
// 	spec.push(&COMMENTS);
// 	spec.push(&NUMBER);
// 	spec.push(&PUNCT);
// 	spec.push(&RETURN);
// 	spec.push(&OPERATOR);
// 	spec.push(&PAREN);
// 	spec.push(&STRING);
// 	spec.push(&WORD);

// 	Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap().

// 	let tokens: Vec<String> = Vec::new();
// 	println!("{}", input);
// }

// enum ResWord {
// 	If,
// 	Else,
// 	True,
// 	False,
// }

// enum Token {
// 	Skip,
// 	Newline,
// 	Number(String),
// 	ReservedWord(ResWord),
// 	// ChildSelector,
// 	// ParentSelector,
// 	Operator(String, i32),
// 	Paren,
// 	Squaren,
// 	Bracket,
// 	Colon,
// 	Key(String),
// 	Ref(String),
// 	Invalid(String),
// }
