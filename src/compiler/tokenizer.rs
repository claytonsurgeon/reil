use lazy_static::lazy_static;
use regex::Regex;
// use std::cmp::Ordering;
// use std::fmt::Display;
// use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
	Skip,
	Newline,
	// Comma,
	// Semicolon,
	Arrow,
	Number,
	Reserved,
	Operator,

	OR,
	AND,
	Equality,
	Relational,
	Additive,
	Multiplicative,
	Exponential,
	Unary,

	Select, // graph.child
	Parent, // graph..parent

	Bool,

	ParenOpen,
	ParenClose,
	SquarenOpen,
	SquarenClose,
	BracketOpen,
	BracketClose,
	Colon,
	String,
	// Key,
	Word,
	Invalid,
}

pub type Token = (Category, String);

pub fn tokenizer(input: &String) -> Vec<(Category, String)> {
	lazy_static! {
		static ref SPEC: Vec<(Category, Regex)> =
			vec![
				// Whitespace
				(Category::Newline, Regex::new(r"^(\n|\r|,|;)+").unwrap()),
				// (Category::Newline, Regex::new(r"^(\n|\r|,|;)+").unwrap()),
				(Category::Skip, Regex::new(r"^[[:blank:]]+").unwrap()),

				// Comments
				(Category::Skip, Regex::new(r"^//.*").unwrap()),
				(Category::Skip, Regex::new(r"^/\*[\s\S]*?\*/").unwrap()),

				// Punctuation
				// (Category::Comma, Regex::new(r"^[,]").unwrap()),
				// (Category::Semicolon, Regex::new(r"^[;]").unwrap()),

				// Function Arrow
				(Category::Arrow, Regex::new(r"^→").unwrap()),
				(Category::Arrow, Regex::new(r"^->").unwrap()),
				// Numbers
				(Category::Number, Regex::new(r"^\-?[0-9]+\.[0-9]*").unwrap()),
				(Category::Number, Regex::new(r"^\-?[0-9]*\.[0-9]+").unwrap()),
				(Category::Number, Regex::new(r"^\-?[0-9]+").unwrap()),

				// Reserved Words
				(Category::Reserved, Regex::new(r"^if\b").unwrap()),
				(Category::Reserved, Regex::new(r"^else\b").unwrap()),
				(Category::Bool, Regex::new(r"^true\b").unwrap()),
				(Category::Bool, Regex::new(r"^false\b").unwrap()),

				// Operators
				(Category::OR, Regex::new(r"^\|").unwrap()),
				(Category::AND, Regex::new(r"^\&").unwrap()),
				(Category::Equality, Regex::new(r"^((==)|(=)|(!=))").unwrap()),
				(Category::Relational, Regex::new(r"^((>=)|(<=)|(>)|(<))").unwrap()),
				(Category::Additive, Regex::new(r"^((-)|(\+))").unwrap()),
				(Category::Multiplicative, Regex::new(r"^((/)|(\*))").unwrap()),
				(Category::Exponential, Regex::new(r"^(\^)").unwrap()),
				(Category::Unary, Regex::new(r"^(!)").unwrap()),

				(Category::Select, Regex::new(r"^[.][.]").unwrap()),
				(Category::Select, Regex::new(r"^[.]").unwrap()),

				(Category::Operator, Regex::new(r"^[.>~<!*=/%÷×·^'∘+-]+").unwrap()),
				// parens
				(Category::ParenOpen, Regex::new(r"^\(").unwrap()),
				(Category::ParenClose, Regex::new(r"^\)").unwrap()),

				(Category::SquarenOpen, Regex::new(r"^\[").unwrap()),
				(Category::SquarenClose, Regex::new(r"^\]").unwrap()),

				(Category::BracketOpen, Regex::new(r"^\{").unwrap()),
				(Category::BracketClose, Regex::new(r"^\}").unwrap()),


				(Category::Colon, Regex::new(r"^:").unwrap()),


				(Category::String, Regex::new(r#"^"[^"]*("|$)"#).unwrap()),


				// (Category::Key, Regex::new(r"^[A-Za-z][A-Za-z0-9]*(?:(\s)*[{])").unwrap()),
				// (Category::Key, Regex::new(r"^[A-Za-z][A-Za-z0-9]*(?:(\s)*[:])").unwrap()),


				(Category::Word, Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap()),

				(Category::Invalid, Regex::new(r"^.").unwrap()),
			];
	}

	let mut tokens: Vec<(Category, String)> = Vec::new();
	let mut cursor = 0;
	// let mut line = 0;
	let length = input.len();

	'outer: while cursor < length {
		for (cat, re) in &SPEC[..] {
			match re.find(&input[cursor..]) {
				Some(mat) => {
					let token_text = &input[cursor..cursor + mat.end()];

					match cat {
						// Category::Key => {
						// 	let m = SPEC[26].1.find(token_text).unwrap().end();
						// 	tokens.push((*Category, token_text[0..m].to_string()))
						// }
						Category::Skip => {}
						_ => tokens.push((*cat, token_text.to_string())),
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
// 	Reserved(ResWord),
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
