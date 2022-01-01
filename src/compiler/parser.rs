// mod tokenizer;
use super::tokenizer;
use tokenizer::Category;
use tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
	UnexpectedEndOfInput,
	UnexpectedToken,
}

struct Aux<'a> {
	cursor: usize,
	tokens: &'a Vec<Token>,
}

impl Aux<'_> {
	fn eat(&mut self, cat: Category) -> Result<Token, ParseError> {
		let x = match self.get(0) {
			Some(t) => {
				if t.0 == cat {
					Ok(t.clone())
				} else {
					Err(ParseError::UnexpectedToken)
				}
			}
			None => Err(ParseError::UnexpectedEndOfInput),
		};

		match x {
			Ok(_) => {
				self.cursor += 1;
			}
			_ => {}
		};

		x
	}

	fn get(&self, offset: usize) -> Option<&Token> {
		if self.cursor + offset < self.tokens.len() {
			Some(&self.tokens[self.cursor + offset])
		} else {
			None
		}
	}

	fn next(&mut self) -> Option<&Token> {
		let next = if self.cursor < self.tokens.len() {
			Some(&self.tokens[self.cursor])
		} else {
			None
		};
		self.cursor += 1;
		next
	}

	fn is(&self, offset: usize, options: &[Category]) -> bool {
		match self.get(offset) {
			Some((cat, _)) => {
				for option in options {
					if *option == *cat {
						return true;
					}
				}
				false
			}
			None => false,
		}
	}

	fn not(&self, offset: usize, stops: &[Category]) -> bool {
		match self.get(offset) {
			Some((cat, _)) => {
				for stop in stops {
					if *stop == *cat {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
}

#[derive(Debug, Clone)]
pub enum AST {
	Graph(String, Vec<AST>),
	Point(String, Box<AST>),
	Error(ParseError),
	Number(String),
	Binary(OPERATOR, Box<AST>, Box<AST>),
	Unary(OPERATOR, Box<AST>),
	Empty,
	Errooooor,
}
#[derive(Debug, Clone)]
pub enum OPERATOR {
	OR,  // |
	AND, // &

	EQ,
	NE, // = == !=

	LT,
	GT, // < >
	LE,
	GE, // <= >=

	ADD,
	SUB, // + -
	MUL,
	DIV, // * /
	EXP, // exponential ^

	NOT,
	NEG,
	ABS, // unary ! - +
}

pub fn parser(tokens: &Vec<(Category, String)>) -> AST {
	let mut aux = Aux { cursor: 0, tokens };

	fn program(tokens: &mut Aux) -> AST {
		AST::Graph(String::from("Program"), point_list(tokens, &[]))
	}

	fn point_list(tokens: &mut Aux, stops: &[Category]) -> Vec<AST> {
		let mut points = vec![];

		while tokens.not(0, stops) {
			points.push(point(tokens, stops));
		}
		points
	}

	fn point(tokens: &mut Aux, stops: &[Category]) -> AST {
		// clear leading newline if present
		let _ = tokens.eat(Category::Newline);

		if tokens.is(1, &[Category::Colon])
			|| tokens.is(1, &[Category::Newline])
				&& tokens.is(2, &[Category::Colon])
		{
			println!("CHECK 2");
			let name = match tokens.eat(Category::Word) {
				Ok(token) => {
					dbg!(&token);
					token
				}
				Err(e) => {
					dbg!(&e);
					tokens.next(); // skip invalid token and return Error Node
					return AST::Error(e);
				}
			};
			// clear leading newline if present
			let _ = tokens.eat(Category::Newline);
			let _ = tokens.eat(Category::Colon);
			// clear leading newline if present
			let _ = tokens.eat(Category::Newline);

			let expression = expre(tokens, stops);

			return AST::Point(name.1.clone(), Box::new(expression));

		// eat expression
		// don't worry about patterns for now
		} else {
		}

		tokens.next();
		AST::Errooooor
	}

	fn expre(tokens: &mut Aux, _stops: &[Category]) -> AST {
		or_expre(tokens)
	}

	fn or_expre(tokens: &mut Aux) -> AST {
		let mut left = and_expre(tokens);

		while tokens.is(0, &[Category::OR]) {
			let _ = tokens.eat(Category::OR);
			left = AST::Binary(
				OPERATOR::OR,
				Box::new(left),
				Box::new(and_expre(tokens)),
			);
		}

		left
	}

	fn and_expre(tokens: &mut Aux) -> AST {
		let mut left = equality_expre(tokens);

		while tokens.is(0, &[Category::AND]) {
			let _ = tokens.eat(Category::AND);
			left = AST::Binary(
				OPERATOR::AND,
				Box::new(left),
				Box::new(equality_expre(tokens)),
			);
		}

		left
	}

	fn equality_expre(tokens: &mut Aux) -> AST {
		let mut left = relational_expre(tokens);

		while tokens.is(0, &[Category::Equality]) {
			let t = tokens.eat(Category::Equality).unwrap();
			let operator = if t.1 == "=" {
				OPERATOR::EQ
			} else if t.1 == "==" {
				OPERATOR::EQ
			} else {
				OPERATOR::NE
			};
			left = AST::Binary(
				operator,
				Box::new(left),
				Box::new(relational_expre(tokens)),
			);
		}

		left
	}

	fn relational_expre(tokens: &mut Aux) -> AST {
		let mut left = additive_expre(tokens);

		while tokens.is(0, &[Category::Relational]) {
			let t = tokens.eat(Category::Relational).unwrap();
			let operator = if t.1 == ">" {
				OPERATOR::GT
			} else if t.1 == "<" {
				OPERATOR::LT
			} else if t.1 == ">=" {
				OPERATOR::GE
			} else {
				OPERATOR::LE
			};
			left = AST::Binary(
				operator,
				Box::new(left),
				Box::new(additive_expre(tokens)),
			);
		}

		left
	}

	fn additive_expre(tokens: &mut Aux) -> AST {
		let mut left = multiplicative_expre(tokens);

		while tokens.is(0, &[Category::Additive]) {
			let t = tokens.eat(Category::Additive).unwrap();
			let operator = if t.1 == "-" {
				OPERATOR::SUB
			} else {
				OPERATOR::ADD
			};
			left = AST::Binary(
				operator,
				Box::new(left),
				Box::new(multiplicative_expre(tokens)),
			);
		}

		left
	}
	fn multiplicative_expre(tokens: &mut Aux) -> AST {
		let mut left = exponential_expre(tokens);

		while tokens.is(0, &[Category::Multiplicative]) {
			let t = tokens.eat(Category::Multiplicative).unwrap();
			let operator = if t.1 == "/" {
				OPERATOR::DIV
			} else {
				OPERATOR::MUL
			};
			left = AST::Binary(
				operator,
				Box::new(left),
				Box::new(exponential_expre(tokens)),
			);
		}

		left
	}

	fn exponential_expre(tokens: &mut Aux) -> AST {
		let mut left = unary_expre(tokens);

		while tokens.is(0, &[Category::Exponential]) {
			let _ = tokens.eat(Category::Exponential);
			left = AST::Binary(
				OPERATOR::EXP,
				Box::new(left),
				Box::new(unary_expre(tokens)),
			);
		}

		left
	}
	fn unary_expre(tokens: &mut Aux) -> AST {
		if tokens.is(0, &[Category::Unary]) {
			let _ = tokens.eat(Category::Unary);
			AST::Unary(OPERATOR::NOT, Box::new(replicate_or_select(tokens)))
		} else if tokens.is(0, &[Category::Additive]) {
			let t = tokens.eat(Category::Additive).unwrap();
			let operator = if t.1 == "-" {
				OPERATOR::NEG
			} else {
				OPERATOR::ABS
			};
			AST::Unary(operator, Box::new(replicate_or_select(tokens)))
		} else {
			replicate_or_select(tokens)
		}
	}

	fn replicate_or_select(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn replicate_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}
	fn select_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	// let eat = |Category: Category| -> Option<&Token>{
	// 	let token = aux.next();

	// 	Some(token)
	// };

	program(&mut aux)
}
