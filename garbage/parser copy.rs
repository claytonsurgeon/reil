// todo: AST functions should probably be methods on "tokens", given that they all taken the token vector as an argument

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
	fn eat_newlines(&mut self) {}
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
	Graph(Vec<AST>),
	Point(String, Box<AST>),
	Error(ParseError),
	Number(String),
	String(String),
	True,
	False,
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

	SELECT,
	PARENT,
}

pub fn parser(tokens: &Vec<(Category, String)>) -> AST {
	let mut aux = Aux { cursor: 0, tokens };

	fn program(tokens: &mut Aux) -> AST {
		AST::Point(
			String::from("Program"),
			Box::new(AST::Graph(point_list(tokens, &[]))),
		)
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
			let name = match tokens.eat(Category::Word) {
				Ok(token) => token,
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
			let _ = tokens.eat(Category::Newline);

			return AST::Point(name.1.clone(), Box::new(expression));

		// eat expression
		// don't worry about patterns for now
		} else {
		}

		let _ = tokens.eat(Category::Newline);

		dbg!(tokens.get(0));
		tokens.next();
		AST::Errooooor
	}

	fn expre(tokens: &mut Aux, _stops: &[Category]) -> AST {
		or_expre(tokens)
	}

	// d: 2*3 + 4*5

	// fn binary_expre(tokens: &mut Aux) {
	// 	let mut left = unary_expre(tokens);

	// 	while tokens.is(0, &[Category::Binary]) {
	// 		let operator = tokens.eat(Category::Binary);
	// 		let right = Box::new(unary_expre(tokens));
	// 		let next = tokens.get(0);

	// 		if next is operator {
	// 			if next_operater.precedence > this_operator.precedence {
	// 				do binary parse
	// 			} else {
	//					do recurse
	// 				//do unary parse
	// 			}
	// 		}
	// 		else {

	// 		}
	// 	}
	// 	left
	// }
	// fn binary_expre(tokens: &mut Aux) {
	// 	let mut left = unary_expre(tokens);

	// 	while tokens.is(0, &[Category::Binary]) {
	// 		let operator = tokens.eat(Category::Binary);
	// 		let right = Box::new(unary_expre(tokens));
	// 		let next = tokens.get(0);

	// 		if next is operator {
	// 			if next_operater.precedence > this_operator.precedence {
	// 				do binary parse
	// 			} else {
	// 				do unary parse
	// 			}
	// 		}
	// 		else {

	// 		}
	// 	}
	// 	left
	// }

	// fn binary_expre(tokens: &mut Aux) {
	// 	let mut left = unary_expre(tokens);

	// 	while tokens.is(0, &[Category::Binary]) {
	// 		let _ = tokens.eat(Category::OR);
	// 		left = AST::Binary(
	// 			OPERATOR::OR,
	// 			Box::new(left),
	// 			Box::new(unary_expre(tokens)),
	// 		);
	// 	}

	// 	left

	// }

	fn or_expre(tokens: &mut Aux) -> AST {
		let mut left = and_expre(tokens);
		while tokens.is(0, &[Category::OR])
			|| tokens.is(0, &[Category::Newline])
				&& tokens.is(1, &[Category::OR])
		{
			let _ = tokens.eat(Category::Newline);
			let _ = tokens.eat(Category::OR);
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let _ = tokens.eat(Category::AND);
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Equality).unwrap();
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Relational).unwrap();
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Additive).unwrap();
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Multiplicative).unwrap();
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let _ = tokens.eat(Category::Exponential);
			let _ = tokens.eat(Category::Newline);
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
			let _ = tokens.eat(Category::Newline);
			let _ = tokens.eat(Category::Unary);
			let _ = tokens.eat(Category::Newline);
			AST::Unary(OPERATOR::NOT, Box::new(replicate_or_select(tokens)))
		} else if tokens.is(0, &[Category::Additive]) {
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Additive).unwrap();
			let _ = tokens.eat(Category::Newline);
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
		if tokens.is(
			1,
			&[
				Category::ParenOpen,
				Category::SquarenOpen,
				Category::BracketOpen,
			],
		) {
			replicate_expre(tokens)
		} else {
			select_expre(tokens)
		}
	}

	fn replicate_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}
	fn select_expre(tokens: &mut Aux) -> AST {
		let mut left = primary_expre(tokens);

		while tokens.is(0, &[Category::Select]) {
			let _ = tokens.eat(Category::Newline);
			let t = tokens.eat(Category::Select).unwrap();
			let _ = tokens.eat(Category::Newline);
			let operator = if t.1 == ".." {
				OPERATOR::PARENT
			} else {
				OPERATOR::SELECT
			};
			left = AST::Binary(
				operator,
				Box::new(left),
				Box::new(selector(tokens)),
			);
		}

		left
	}

	fn selector(tokens: &mut Aux) -> AST {
		if tokens.is(0, &[Category::ParenOpen]) {
			tuple_selector(tokens)
		} else if tokens.is(0, &[Category::SquarenOpen]) {
			array_selector(tokens)
		} else if tokens.is(0, &[Category::BracketOpen]) {
			graph_selector(tokens)
		} else {
			point_selector(tokens)
		}
	}

	fn point_selector(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn tuple_selector(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn array_selector(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn graph_selector(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn primary_expre(tokens: &mut Aux) -> AST {
		dbg!(tokens.get(0));
		let r = if tokens.is(0, &[Category::ParenOpen]) {
			tuple_expre(tokens)
		} else if tokens.is(0, &[Category::SquarenOpen]) {
			array_expre(tokens)
		} else if tokens.is(0, &[Category::BracketOpen]) {
			graph_expre(tokens)
		} else if tokens.is(0, &[Category::Word]) {
			point_expre(tokens)
		} else {
			literal_expre(tokens)
		};

		let _ = tokens.eat(Category::Newline);

		r
	}

	fn point_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn tuple_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn array_expre(tokens: &mut Aux) -> AST {
		tokens.next();
		AST::Empty
	}

	fn graph_expre(tokens: &mut Aux) -> AST {
		let _ = tokens.eat(Category::BracketOpen);
		let g = AST::Graph(point_list(tokens, &[Category::BracketClose]));
		let _ = tokens.eat(Category::BracketClose);
		g
	}

	fn literal_expre(tokens: &mut Aux) -> AST {
		if tokens.is(0, &[Category::Number]) {
			let t = tokens.eat(Category::Number).unwrap();
			AST::Number(t.1.clone())
		} else if tokens.is(0, &[Category::String]) {
			let t = tokens.eat(Category::String).unwrap();
			AST::String(t.1.clone())
		} else {
			match tokens.eat(Category::Bool) {
				Ok(t) => {
					if t.1 == "true" {
						AST::True
					} else {
						AST::False
					}
				}
				Err(e) => AST::Error(e),
			}
		}
	}

	// let t = tokens.eat(Category::Select).unwrap();
	// let operator = if t.1 == ".." {
	// 	OPERATOR::PARENT
	// } else {
	// 	OPERATOR::SELECT
	// };
	// let mut left = exponential_expre(tokens);

	// while tokens.is(0, &[Category::Multiplicative]) {
	// 	let t = tokens.eat(Category::Multiplicative).unwrap();
	// 	let operator = if t.1 == "/" {
	// 		OPERATOR::DIV
	// 	} else {
	// 		OPERATOR::MUL
	// 	};
	// 	left = AST::Binary(
	// 		operator,
	// 		Box::new(left),
	// 		Box::new(exponential_expre(tokens)),
	// 	);
	// }

	// left

	program(&mut aux)
}
