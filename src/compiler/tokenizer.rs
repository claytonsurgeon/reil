use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
	Invalid,
	Skip,
	Stop,
	//
	// Colon,
	Arrow,
	Binary,
	Unary,
	Select,
	//
	Word,
	Reserved,
	//
	Bool,
	String,
	Number,
	//
	Paren,
	Squaren,
	Bracket,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
	Invalid,
	Skip,
	Newline,
	Comma,
	//
	Colon,
	Pattern,
	Arrow,
	//
	Or,
	And,
	Add,
	Sub,
	Mul,
	Div,
	Exp,
	Not,
	//
	Eq,
	Ne,
	Gt,
	Lt,
	Ge,
	Le,
	//
	Select,
	Parent,
	//
	Word,
	Reserved,
	//
	True,
	False,
	String,
	Number,
	//
	ParenLF,
	ParenRT,
	SquarenLF,
	SquarenRT,
	BracketLF,
	BracketRT,
}

// 0 is lowest precendence, 255 is highest
// 255 is used for non binary operations

pub fn precedence(token: &Token) -> u8 {
	use Name::*;
	match token.1 {
		Colon => 0,
		Arrow => 0,
		Pattern => 1,
		//
		Or => 2,
		And => 3,

		Eq => 4,
		Ne => 4,

		Gt => 5,
		Lt => 5,
		Ge => 5,
		Le => 5,

		Add => 6,
		Sub => 6,
		Mul => 7,
		Div => 7,
		Exp => 8,
		// Not => 9,
		//
		//
		Select => 9,
		Parent => 9,
		_ => 0,
	}
}

pub type Token = (Class, Name, String);

pub fn tokenizer(input: &String) -> Vec<Token> {
	lazy_static! {
		static ref SPEC: Vec<(Class, Name, Regex)> =
			vec![
				(Class::Stop, Name::Newline, Regex::new(r"^(\n|\r)+").unwrap()),
				(Class::Stop, Name::Comma, Regex::new(r"^(,|;)+").unwrap()),
				(Class::Skip, Name::Skip, Regex::new(r"^[[:blank:]]+").unwrap()),
				// Comments
				(Class::Skip, Name::Skip, Regex::new(r"^//.*").unwrap()),
				(Class::Skip, Name::Skip, Regex::new(r"^/\*[\s\S]*?\*/").unwrap()),

				// Function Arrow
				(Class::Arrow, Name::Arrow, Regex::new(r"^(->|→)").unwrap()),
				// Numbers
				(Class::Number, Name::Number, Regex::new(r"^\-?[0-9]+\.[0-9]*").unwrap()),
				(Class::Number, Name::Number, Regex::new(r"^\-?[0-9]*\.[0-9]+").unwrap()),
				(Class::Number, Name::Number, Regex::new(r"^\-?[0-9]+").unwrap()),

				// Reserved Words
				(Class::Reserved, Name::Reserved, Regex::new(r"^if\b").unwrap()),
				(Class::Reserved, Name::Reserved, Regex::new(r"^else\b").unwrap()),

				(Class::Bool, Name::True, Regex::new(r"^true\b").unwrap()),
				(Class::Bool, Name::False, Regex::new(r"^false\b").unwrap()),
				// Operators
				(Class::Binary, Name::Colon, Regex::new(r"^[:]").unwrap()),
				(Class::Binary, Name::Pattern, Regex::new(r"^[~]").unwrap()),
				(Class::Binary, Name::Or, Regex::new(r"^[|]").unwrap()),
				(Class::Binary, Name::And, Regex::new(r"^[&]").unwrap()),
				(Class::Binary, Name::Eq, Regex::new(r"^((==)|(=))").unwrap()),
				(Class::Binary, Name::Ne, Regex::new(r"^(!=)").unwrap()),
				(Class::Binary, Name::Ge, Regex::new(r"^(>=)").unwrap()),
				(Class::Binary, Name::Le, Regex::new(r"^(<=)").unwrap()),
				(Class::Binary, Name::Lt, Regex::new(r"^(>)").unwrap()),
				(Class::Binary, Name::Le, Regex::new(r"^(<)").unwrap()),
				(Class::Binary, Name::Add, Regex::new(r"^(\+)").unwrap()),
				(Class::Binary, Name::Sub, Regex::new(r"^(-)").unwrap()),
				(Class::Binary, Name::Mul, Regex::new(r"^(\*)").unwrap()),
				(Class::Binary, Name::Div, Regex::new(r"^(/)").unwrap()),
				(Class::Binary, Name::Exp, Regex::new(r"^(\^)").unwrap()),
				(Class::Unary, Name::Not, Regex::new(r"^(!)").unwrap()),
				(Class::Select, Name::Parent, Regex::new(r"^[.][.]").unwrap()),
				(Class::Select, Name::Select, Regex::new(r"^[.]").unwrap()),


				// (Class::Operator, Regex::new(r"^[.>~<!*=/%÷×·^'∘+-]+").unwrap()),
				// parens
				(Class::Paren, Name::ParenLF, Regex::new(r"^\(").unwrap()),
				(Class::Paren, Name::ParenRT, Regex::new(r"^\)").unwrap()),

				(Class::Squaren, Name::SquarenLF, Regex::new(r"^\[").unwrap()),
				(Class::Squaren, Name::SquarenRT, Regex::new(r"^\]").unwrap()),

				(Class::Bracket, Name::BracketLF, Regex::new(r"^\{").unwrap()),
				(Class::Bracket, Name::BracketRT, Regex::new(r"^\}").unwrap()),


				(Class::String, Name::String, Regex::new(r#"^"[^"]*("|$)"#).unwrap()),


				(Class::Word, Name::Word, Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap()),

				(Class::Invalid, Name::Word, Regex::new(r"^.").unwrap()),
			];
	}

	let mut tokens: Vec<Token> = Vec::new();
	let mut cursor = 0;
	// let mut line = 0;
	let length = input.len();

	'outer: while cursor < length {
		for (class, name, re) in &SPEC[..] {
			match re.find(&input[cursor..]) {
				Some(mat) => {
					let token_text = &input[cursor..cursor + mat.end()];

					match class {
						Class::Skip => {}
						_ => {
							tokens.push((*class, *name, token_text.to_string()))
						}
					}

					cursor += mat.end();
					continue 'outer;
				}
				None => {}
			}
		}
	}

	let mut filtered_tokens: Vec<Token> = Vec::new();
	let mut skip_initial_newlines = true;
	let mut last_token_was_newline = false;
	let mut last_token_was_comma = false;
	let mut last_token_was_operator = false;
	for token in tokens {
		match token {
			(Class::Stop, Name::Comma, _) => {
				if last_token_was_newline {
					filtered_tokens.pop();
				}
				filtered_tokens.push(token);
				last_token_was_operator = false;
				last_token_was_comma = true;
				last_token_was_newline = false;
			}
			(Class::Stop, Name::Newline, _) => {
				if !last_token_was_operator
					&& !last_token_was_comma
					&& !last_token_was_newline
					&& !skip_initial_newlines
				{
					filtered_tokens.push(token);
				}
				last_token_was_operator = false;
				last_token_was_comma = false;
				last_token_was_newline = true;
			}
			(Class::Binary, _, _)
			| (Class::Unary, _, _)
			| (Class::Select, _, _)
			| (Class::Arrow, _, _) => {
				if last_token_was_newline {
					filtered_tokens.pop();
				}
				filtered_tokens.push(token);
				last_token_was_operator = true;
				last_token_was_comma = false;
				last_token_was_newline = false;
			}
			// (Class::Unary, _, _) | (Class::Select, _, _) => {
			// 	filtered_tokens.push(token);
			// 	last_token_was_operator = true;
			// 	last_token_was_comma = false;
			// 	last_token_was_newline = false;
			// }
			_ => {
				filtered_tokens.push(token);
				last_token_was_operator = false;
				last_token_was_comma = false;
				last_token_was_newline = false;
				skip_initial_newlines = false;
			}
		}
	}

	filtered_tokens
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
