use super::tokenizer::{precedence, Class, Name, Token};
use std::cell::RefCell;

pub struct Tokens<'a> {
	cursor: RefCell<usize>,
	tokens: &'a Vec<Token>,
}

pub fn parser(tokens: &Vec<Token>) -> AST {
	let cursor = Tokens {
		cursor: RefCell::new(0),
		tokens,
	};
	cursor.program()
}

#[derive(Debug, Clone)]
pub enum AST {
	Point(String, Box<AST>),
	Graph(Vec<AST>),
	Array(Vec<AST>),
	Tuple(Vec<AST>),

	Bool(bool),
	String(String),
	Number(String),
	Error(String),
	Return(Box<AST>),
	MisplacedExpression(Box<AST>),

	Ref(String),
	Rep(String, Box<AST>), // replicate

	Binary(Name, Box<AST>, Box<AST>),
	Unary(Name, Box<AST>, Box<AST>),
	Nothing,
}

fn greater_precedence(next_token: &Token, token: &Token) -> bool {
	// dbg!(precedence(next_token.1), precedence(token.1));
	precedence(next_token) > precedence(token)
}

impl Tokens<'_> {
	fn program(&self) -> AST {
		AST::Point(
			String::from("Program"), // should be file name
			Box::new(AST::Graph(self.point_list(&[]))),
		)
	}

	fn point_list(&self, stops: &[Class]) -> Vec<AST> {
		let mut points = vec![];

		self.clear_stops();
		while self.until(0, stops) {
			points.push(self.point(stops));
		}
		self.clear_stops();

		match points.last() {
			Some(AST::MisplacedExpression(expr)) => {
				let last = AST::Return(expr.clone());
				points.pop();
				points.push(last);
			}
			_ => {}
		}

		points
	}

	fn point(&self, _stops: &[Class]) -> AST {
		// assume a naked expresssion is an error, "disconnected point", if it's the last expression in graph, make it the return point

		self.clear_stops();
		if self.is(0, Name::Word) && self.is(1, Name::Colon) {
			let name = self.word_val();
			self.eat_any(); // eat ':'

			let point = AST::Point(name, Box::new(self.expression()));
			self.clear_stops();
			point
		} else {
			let expression = self.expression();
			self.clear_stops();
			AST::MisplacedExpression(Box::new(expression))
		}
	}

	fn expression(&self) -> AST {
		self.binary_expr()
	}

	fn binary_expr(&self) -> AST {
		let mut left = self.unary_expr();
		let prec: u8 = if self.of(0, Class::Binary) {
			precedence(self.get(0).unwrap())
		} else {
			255
		};

		while self.of(0, Class::Binary)
			&& precedence(self.get(0).unwrap()) == prec
		{
			let operator = self.eat(Class::Binary).unwrap();

			if self.of(1, Class::Binary) {
				let next_operator = self.get(1).unwrap();
				if greater_precedence(next_operator, operator) {
					left = AST::Binary(
						operator.1,
						Box::new(left),
						Box::new(self.binary_expr()),
					);
				// right = self.binary_expr();
				} else {
					left = AST::Binary(
						operator.1,
						Box::new(left),
						Box::new(self.unary_expr()),
					);
					continue;
				}
			} else {
				left = AST::Binary(
					operator.1,
					Box::new(left),
					Box::new(self.unary_expr()),
				);
			}
		}
		left
	}

	// fn binary_expr(&self) -> AST {
	// 	let mut left = self.unary_expr();

	// 	while self.of(0, Class::Binary) {
	// 		let operator = self.eat(Class::Binary).unwrap();

	// 		if self.of(1, Class::Binary) {
	// 			let next_operator = self.get(1).unwrap();
	// 			// println!(
	// 			// 	"{:?} > {:?}",
	// 			// 	precedence(next_operator.1),
	// 			// 	precedence(operator.1)
	// 			// );
	// 			// dbg!(greater_precedence(next_operator, operator));
	// 			let right = if greater_precedence(next_operator, operator) {
	// 				self.binary_expr()
	// 			} else {
	// 				self.unary_expr()
	// 			};
	// 			left =
	// 				AST::Binary(operator.1, Box::new(left), Box::new(right));
	// 		} else {
	// 			left = AST::Binary(
	// 				operator.1,
	// 				Box::new(left),
	// 				Box::new(self.unary_expr()),
	// 			);
	// 		}
	// 	}
	// 	left
	// }

	fn unary_expr(&self) -> AST {
		self.literal()
	}

	fn word_val(&self) -> String {
		match self.eat(Class::Word) {
			Ok(t) => t.2.clone(),
			Err(e) => e,
		}
	}

	fn literal(&self) -> AST {
		match self.get(0) {
			Some((class, _name, _token)) => match class {
				Class::Bool => self.bool(),
				Class::Number => self.number(),
				Class::String => self.string(),
				_ => self.unexpected_token(),
			},
			None => AST::Error("UnexpectedEndOfInput".to_string()),
		}
	}

	fn unexpected_token(&self) -> AST {
		match self.next() {
			Some((class, name, token)) => AST::Error(format!(
				"UnexpectedToken:  `{}`  {:?} {:?}",
				token.clone(),
				class,
				name
			)),
			None => AST::Error("UnexpectedEndOfInput".to_string()),
		}
	}

	fn bool(&self) -> AST {
		match self.eat(Class::Bool) {
			Ok(t) => AST::Bool(if t.1 == Name::True { true } else { false }),
			Err(e) => AST::Error(e),
		}
	}

	fn number(&self) -> AST {
		match self.eat(Class::Number) {
			Ok(t) => AST::Number(t.2.clone()),
			Err(e) => AST::Error(e),
		}
	}

	fn string(&self) -> AST {
		match self.eat(Class::String) {
			Ok(t) => AST::String(t.2.clone()),
			Err(e) => AST::Error(e),
		}
	}
}

// static precendence: [Name; 1] = [
// 	Name::Add
// ];

/*





*/
impl Tokens<'_> {
	fn eat(&self, class: Class) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if t.0 == class {
					Ok(t)
				} else {
					Err(format!("UnexpectedToken: {}", t.2))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}
	fn eat_any(&self) -> AST {
		*self.cursor.borrow_mut() += 1;
		AST::Nothing
	}

	fn _eat_unwrap(&self) -> &Token {
		self.get(0).unwrap()
	}

	fn clear_newlines(&self) {
		while self.is(0, Name::Newline) {
			*self.cursor.borrow_mut() += 1;
		}
	}
	fn clear_stops(&self) {
		while self.of(0, Class::Stop) {
			*self.cursor.borrow_mut() += 1;
		}
	}
	fn next(&self) -> Option<&Token> {
		if *self.cursor.borrow() < self.tokens.len() {
			let next_token = Some(&self.tokens[*self.cursor.borrow()]);
			*self.cursor.borrow_mut() += 1;
			next_token
		} else {
			None
		}
	}
	fn get(&self, offset: usize) -> Option<&Token> {
		if *self.cursor.borrow() + offset < self.tokens.len() {
			Some(&self.tokens[*self.cursor.borrow() + offset])
		} else {
			None
		}
	}

	fn is(&self, offset: usize, stop: Name) -> bool {
		match self.get(offset) {
			Some((_, name, _)) => *name == stop,
			None => false,
		}
	}

	fn of(&self, offset: usize, stop: Class) -> bool {
		match self.get(offset) {
			Some((class, _, _)) => *class == stop,
			None => false,
		}
	}

	fn _any(&self, offset: usize, stops: &[Name]) -> bool {
		for stop in stops {
			if self.is(offset, *stop) {
				return true;
			}
		}
		false
	}

	fn _any_of(&self, offset: usize, stops: &[Class]) -> bool {
		for stop in stops {
			if self.of(offset, *stop) {
				return true;
			}
		}
		false
	}

	fn _not(&self, offset: usize, stop: Name) -> bool {
		!self.is(offset, stop)
	}

	fn _not_of(&self, offset: usize, stop: Class) -> bool {
		!self.of(offset, stop)
	}

	// fn not_any(&self, offset: usize, stops: &[Class]) -> bool {
	// 	let x_stops = [stops, &[Class::Stop]].concat();
	// 	!self.any(offset, stops)
	// }

	fn until(&self, offset: usize, stops: &[Class]) -> bool {
		match self.get(offset) {
			Some((class, _, _)) => {
				if *class == Class::Stop {
					return false;
				}
				for stop in stops {
					if *class == *stop {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
}
