use super::tokenizer::{Kind, Meta, Name, Of, Token};
use std::cell::RefCell;

pub struct Tokens<'a> {
	cursor: RefCell<usize>,
	tokens: &'a Vec<Token>,
}

pub fn parser(tokens: &Vec<Token>) -> Result<AST, String> {
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
	Return(Box<AST>),

	Ref(String),
	Rep(Box<AST>, Box<AST>), // replicate

	Binary(Name, Box<AST>, Box<AST>),
	Unary(Name, Box<AST>),
	Nothing,
}

// type ResAST = Result<AST, String>;

impl Tokens<'_> {
	fn program(&self) -> Result<AST, String> {
		Ok(AST::Point(
			String::from("Program"), // should be file name
			Box::new(AST::Graph(self.point_list(&[])?)),
		))
	}

	fn point_list(&self, stops: &[Name]) -> Result<Vec<AST>, String> {
		let mut points: Vec<AST> = vec![];

		self.clear_stops();
		while self.until(0, stops) {
			points.push(self.point()?);
			self.clear_stops();
		}

		// Ok(points)
		Ok(points)
	}

	fn point(&self) -> Result<AST, String> {
		// assume a naked expresssion is an error, "disconnected point", if it's the last expression in graph, make it the return point

		self.clear_stops();
		if self.is(0, Name::Key) {
			let text = &self.eat(Name::Key)?.meta.text;
			let key_text = text[..text.len() - 1].to_string().clone();

			let point = AST::Point(key_text, Box::new(self.expression()?));
			// let point = AST::Point(key_text, Box::new(AST::Nothing));
			self.clear_stops();
			Ok(point)
		} else {
			self.eat(Name::Arrow)?;
			let expression = self.expression()?;
			self.clear_stops();
			Ok(expression)
		}
	}

	fn expression(&self) -> Result<AST, String> {
		self.or_exp()
	}

	fn or_exp(&self) -> Result<AST, String> {
		let mut left = self.and_exp()?;
		while self.is(0, Name::Or) {
			self.eat(Name::Or)?;
			left = AST::Binary(
				Name::Or,
				Box::new(left),
				Box::new(self.and_exp()?),
			);
		}

		Ok(left)
	}

	fn and_exp(&self) -> Result<AST, String> {
		let mut left = self.equality_exp()?;
		while self.is(0, Name::And) {
			self.eat(Name::And)?;
			left = AST::Binary(
				Name::And,
				Box::new(left),
				Box::new(self.equality_exp()?),
			);
		}

		Ok(left)
	}

	fn equality_exp(&self) -> Result<AST, String> {
		let mut left = self.relation_exp()?;
		while self.any(0, &[Name::Eq, Name::Ne]) {
			let t = self.eat_of(Kind::Binary)?;
			left = AST::Binary(
				t.of.name,
				Box::new(left),
				Box::new(self.relation_exp()?),
			);
		}

		Ok(left)
	}

	fn relation_exp(&self) -> Result<AST, String> {
		let mut left = self.additive_exp()?;
		while self.any(0, &[Name::Gt, Name::Ge, Name::Lt, Name::Le]) {
			let t = self.eat_of(Kind::Binary)?;
			left = AST::Binary(
				t.of.name,
				Box::new(left),
				Box::new(self.additive_exp()?),
			);
		}

		Ok(left)
	}

	fn additive_exp(&self) -> Result<AST, String> {
		let mut left = self.multiplicative_exp()?;
		while self.any(0, &[Name::Add, Name::Sub]) {
			let t = self.eat_of(Kind::Binary)?;
			left = AST::Binary(
				t.of.name,
				Box::new(left),
				Box::new(self.multiplicative_exp()?),
			);
		}

		Ok(left)
	}

	fn multiplicative_exp(&self) -> Result<AST, String> {
		let mut left = self.exponential_exp()?;
		while self.any(0, &[Name::Mul, Name::Div]) {
			let t = self.eat_of(Kind::Binary).unwrap();
			left = AST::Binary(
				t.of.name,
				Box::new(left),
				Box::new(self.exponential_exp()?),
			);
		}

		Ok(left)
	}

	fn exponential_exp(&self) -> Result<AST, String> {
		let mut left = self.unary_exp()?;
		while self.is(0, Name::Exp) {
			self.eat(Name::Exp)?;
			left = AST::Binary(
				Name::Exp,
				Box::new(left),
				Box::new(self.unary_exp()?),
			);
		}

		Ok(left)
	}

	fn unary_exp(&self) -> Result<AST, String> {
		if self.any(0, &[Name::Add, Name::Sub, Name::Not]) {
			let operator = self.eats(&[Name::Add, Name::Sub, Name::Not])?;
			Ok(AST::Unary(operator.of.name, Box::new(self.unary_exp()?)))
		} else {
			// self.replicate_or_select()?
			// Ok(AST::Nothing)
			Ok(self.literal()?)
		}
	}

	fn literal(&self) -> Result<AST, String> {
		match self.get(0) {
			Some(t) => match t.of.kind {
				Kind::Bool => self.bool(),
				Kind::Number => self.number(),
				Kind::String => self.string(),
				_ => Err(format!(
					"UnexpectedToken: {:?} on line {}",
					t.meta.text.clone(),
					t.meta.line
				)),
			},
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}

	fn bool(&self) -> Result<AST, String> {
		let t = self.eat_of(Kind::Bool)?;
		Ok(AST::Bool(if t.of.name == Name::True {
			true
		} else {
			false
		}))
	}

	fn number(&self) -> Result<AST, String> {
		let t = self.eat_of(Kind::Number)?;
		Ok(AST::Number(t.meta.text.clone()))
	}

	fn string(&self) -> Result<AST, String> {
		let t = self.eat_of(Kind::String)?;
		Ok(AST::String(t.meta.text.clone()))
	}
}

// static precendence: [Name; 1] = [
// 	Name::Add
// ];

/*





*/
impl Tokens<'_> {
	fn eat(&self, name: Name) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if t.of.name == name {
					Ok(t)
				} else {
					Err(format!(
						"UnexpectedToken: {:?} on line {}\nExpected token of name: {:?}",
						t.meta.text, t.meta.line, t.of.name
					))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}
	fn eat_of(&self, kind: Kind) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if t.of.kind == kind {
					Ok(t)
				} else {
					Err(format!(
						"UnexpectedToken: {:?} on line {}\nExpected token of kind: {:?}",
						t.meta.text, t.meta.line, t.of.kind
					))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}

	fn eats(&self, names: &[Name]) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if self.any(0, names) {
					Ok(t)
				} else {
					Err(format!(
						"UnexpectedToken: {:?} on line {}\nExpected token of name: {:?}",
						t.meta.text, t.meta.line, t.of.name
					))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}

	fn clear_stops(&self) {
		while self.of(0, Kind::Stop) {
			*self.cursor.borrow_mut() += 1;
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
			Some(t) => t.of.name == stop,
			None => false,
		}
	}

	fn of(&self, offset: usize, stop: Kind) -> bool {
		match self.get(offset) {
			Some(t) => t.of.kind == stop,
			None => false,
		}
	}
	fn any(&self, offset: usize, names: &[Name]) -> bool {
		for name in names {
			if self.is(offset, *name) {
				return true;
			}
		}
		false
	}

	fn until(&self, offset: usize, stops: &[Name]) -> bool {
		match self.get(offset) {
			Some(t) => {
				for stop in stops {
					if t.of.name == *stop {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
}
