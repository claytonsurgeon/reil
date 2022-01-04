use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
	Invalid,
	Skip,
	Stop,
	//
	// Colon,
	Binary,
	Unary,
	Select,
	//
	Label,
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
	Key,
	Ref,
	Arrow,
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
	match token.of.name {
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

pub struct Meta {
	pub line: u32,
	pub text: String,
}
pub struct Of {
	pub kind: Kind,
	pub name: Name,
}
pub struct Token {
	pub of: Of,
	pub meta: Meta,
}

pub fn tokenizer(input: &String) -> Vec<Token> {
	lazy_static! {
		static ref SPEC: Vec<(Kind, Name, Regex)> =
			vec![
				(Kind::Stop, Name::Newline, Regex::new(r"^\n").unwrap()),
				(Kind::Skip, Name::Skip, Regex::new(r"^\r").unwrap()),
				(Kind::Stop, Name::Comma, Regex::new(r"^(,|;)+").unwrap()),
				(Kind::Skip, Name::Skip, Regex::new(r"^[[:blank:]]+").unwrap()),
				// Comments
				(Kind::Skip, Name::Skip, Regex::new(r"^//.*").unwrap()),
				(Kind::Skip, Name::Skip, Regex::new(r"^/\*[\s\S]*?\*/").unwrap()),

				// Labels
				// should support redefining operators, just the basics for now
				(Kind::Label, Name::Key, Regex::new(r"^[*/+-]:").unwrap()),
				(Kind::Label, Name::Key, Regex::new(r"^[A-Za-z][A-Za-z0-9]*:").unwrap()),

				(Kind::Label, Name::Ref, Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap()),
				(Kind::Label, Name::Arrow, Regex::new(r"^(->|→)").unwrap()),

				// Numbers
				(Kind::Number, Name::Number, Regex::new(r"^\-?[0-9]+\.[0-9]*").unwrap()),
				(Kind::Number, Name::Number, Regex::new(r"^\-?[0-9]*\.[0-9]+").unwrap()),
				(Kind::Number, Name::Number, Regex::new(r"^\-?[0-9]+").unwrap()),

				// Reserved Words
				(Kind::Reserved, Name::Reserved, Regex::new(r"^if\b").unwrap()),
				(Kind::Reserved, Name::Reserved, Regex::new(r"^else\b").unwrap()),

				(Kind::Bool, Name::True, Regex::new(r"^true\b").unwrap()),
				(Kind::Bool, Name::False, Regex::new(r"^false\b").unwrap()),
				// Operators
				(Kind::Binary, Name::Colon, Regex::new(r"^[:]").unwrap()),
				(Kind::Binary, Name::Pattern, Regex::new(r"^[~]").unwrap()),
				(Kind::Binary, Name::Or, Regex::new(r"^[|]").unwrap()),
				(Kind::Binary, Name::And, Regex::new(r"^[&]").unwrap()),
				(Kind::Binary, Name::Eq, Regex::new(r"^((==)|(=))").unwrap()),
				(Kind::Binary, Name::Ne, Regex::new(r"^(!=)").unwrap()),
				(Kind::Binary, Name::Ge, Regex::new(r"^(>=)").unwrap()),
				(Kind::Binary, Name::Le, Regex::new(r"^(<=)").unwrap()),
				(Kind::Binary, Name::Lt, Regex::new(r"^(>)").unwrap()),
				(Kind::Binary, Name::Le, Regex::new(r"^(<)").unwrap()),
				(Kind::Binary, Name::Add, Regex::new(r"^(\+)").unwrap()),
				(Kind::Binary, Name::Sub, Regex::new(r"^(-)").unwrap()),
				(Kind::Binary, Name::Mul, Regex::new(r"^(\*)").unwrap()),
				(Kind::Binary, Name::Div, Regex::new(r"^(/)").unwrap()),
				(Kind::Binary, Name::Exp, Regex::new(r"^(\^)").unwrap()),
				(Kind::Unary, Name::Not, Regex::new(r"^(!)").unwrap()),
				(Kind::Select, Name::Parent, Regex::new(r"^[.][.]").unwrap()),
				(Kind::Select, Name::Select, Regex::new(r"^[.]").unwrap()),


				// (Kind::Operator, Regex::new(r"^[.>~<!*=/%÷×·^'∘+-]+").unwrap()),
				// parens
				(Kind::Paren, Name::ParenLF, Regex::new(r"^\(").unwrap()),
				(Kind::Paren, Name::ParenRT, Regex::new(r"^\)").unwrap()),

				(Kind::Squaren, Name::SquarenLF, Regex::new(r"^\[").unwrap()),
				(Kind::Squaren, Name::SquarenRT, Regex::new(r"^\]").unwrap()),

				(Kind::Bracket, Name::BracketLF, Regex::new(r"^\{").unwrap()),
				(Kind::Bracket, Name::BracketRT, Regex::new(r"^\}").unwrap()),


				(Kind::String, Name::String, Regex::new(r#"^"[^"]*("|$)"#).unwrap()),



				(Kind::Invalid, Name::Invalid, Regex::new(r"^.").unwrap()),
			];
	}

	let mut tokens: Vec<Token> = Vec::new();
	let mut cursor = 0;
	let mut line = 1;
	let length = input.len();

	let mut skip_initial_newlines = true;
	let mut last_token_was_newline = false;
	let mut last_token_was_comma = false;
	let mut last_token_was_operator = false;

	'outer: while cursor < length {
		for (kind, name, re) in &SPEC[..] {
			match re.find(&input[cursor..]) {
				Some(mat) => {
					let token_text = &input[cursor..cursor + mat.end()];

					let t = Token {
						of: Of {
							kind: *kind,
							name: *name,
						},
						meta: Meta {
							line,
							text: token_text.to_string(),
						},
					};

					match (kind, name) {
						(Kind::Skip, _) => {}
						(Kind::Stop, Name::Comma) => {
							if last_token_was_newline {
								tokens.pop();
							}
							if !last_token_was_comma {
								tokens.push(t);
							}
							last_token_was_operator = false;
							last_token_was_comma = true;
							last_token_was_newline = false;
						}
						(Kind::Stop, Name::Newline) => {
							if !last_token_was_operator
								&& !last_token_was_comma && !last_token_was_newline
								&& !skip_initial_newlines
							{
								tokens.push(t);
							}
							last_token_was_operator = false;
							last_token_was_comma = false;
							last_token_was_newline = true;
							line += 1;
						}
						(Kind::Binary, _)
						| (Kind::Unary, _)
						| (Kind::Select, _)
						| (Kind::Label, Name::Key)
						| (Kind::Label, Name::Arrow) => {
							if last_token_was_newline {
								tokens.pop();
							}
							tokens.push(t);
							last_token_was_operator = true;
							last_token_was_comma = false;
							last_token_was_newline = false;
						}
						_ => {
							tokens.push(t);

							last_token_was_operator = false;
							last_token_was_comma = false;
							last_token_was_newline = false;
							skip_initial_newlines = false;
						}
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
