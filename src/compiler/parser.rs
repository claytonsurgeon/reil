use super::tokenizer::{Kind, Name, Token};
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
pub enum Number {
	Integer,
	Decimal,
	Boolean,
	I8,
	I16,
	I32,
	I64,
	I128,
	U8,
	U16,
	U32,
	U64,
	U128,
	F32,
	F64,
	F128,
}

fn name_to_number(name: Name) -> Number {
	match name {
		Name::Decimal => Number::Decimal,
		Name::Integer => Number::Integer,
		Name::Boolean => Number::Boolean,
		_ => panic!("name_to_number should be infallible"),
	}
}

#[derive(Debug, Clone)]
pub enum AST {
	Nothing,
	//
	Number(Number, String),
	String(String),

	Graph(Vec<AST>),
	Array(Vec<AST>),
	Tuple(Vec<AST>),
	Point(String, u16, bool, Box<AST>), // Label, Index, isReturn, Value

	Op2(Name, Box<AST>, Box<AST>),
	Op1(Name, Box<AST>),

	Ref(String),
	Arg(Box<AST>),
	Rep(Box<AST>, Box<AST>),
	//
}

// pub enum AST {
// 	Point(String, Box<AST>),
// 	Graph(Vec<AST>),
// 	Array(Vec<AST>),
// 	Tuple(Vec<AST>),

// 	Bool(bool),
// 	String(String),
// 	Integer(String),
// 	Decimal(String),
// 	Return(Box<AST>),
// 	Arg(Box<AST>),

// 	Ref(String),
// 	Refs(Vec<AST>),
// 	Rep(Box<AST>, Box<AST>), // replicate

// 	Binary(Name, Box<AST>, Box<AST>),
// 	Unary(Name, Box<AST>),
// 	Nothing,
// }

// type ResAST = Result<AST, String>;

impl Tokens<'_> {
	fn program(&self) -> Result<AST, String> {
		Ok(AST::Point(
			String::from("Program"), // should be file name
			0,
			false,
			Box::new(AST::Graph(self.point_list(&[])?)),
		))
	}

	fn point_list(&self, stops: &[Name]) -> Result<Vec<AST>, String> {
		let mut points: Vec<AST> = vec![];

		self.clear_stops();
		let mut index = 0;
		while self.until(0, stops) {
			let (ast, ispoint) = self.point(index)?;
			index += if ispoint { 1 } else { 0 };
			points.push(ast);
			self.clear_stops();
		}

		// Ok(points)
		Ok(points)
	}

	fn point(&self, index: u16) -> Result<(AST, bool), String> {
		self.clear_stops();

		if self.is(0, Name::Colon) {
			let connection = self.unary_exp()?;
			self.clear_stops();
			return Ok((connection, false));
		}

		let isreturn = self.is(0, Name::Arrow);
		if isreturn {
			self.eat(Name::Arrow)?;
		}

		let label = if self.is(0, Name::Key) {
			let text = &self.eat(Name::Key)?.meta.text;
			text[..text.len() - 1].to_string().clone()
		} else {
			String::new()
		};

		let expression = Box::new(if self.is(0, Name::Key) {
			AST::Nothing
		} else {
			let x = self.expression()?;
			self.clear_stops();
			x
		});

		Ok((AST::Point(label, index, isreturn, expression), true))
	}

	fn expression(&self) -> Result<AST, String> {
		// self.or_exp()
		self.pattern_exp()
	}

	fn pattern_exp(&self) -> Result<AST, String> {
		let mut left = self.or_exp()?;

		// if left is type replicant, then check for pattern operator
		match left {
			AST::Array(_)
			| AST::Tuple(_)
			| AST::Graph(_)
			| AST::Rep(_, _) => {
				if self.is(0, Name::Pattern) {
					self.eat(Name::Pattern)?;
					left = AST::Op2(
						Name::Pattern,
						Box::new(left),
						Box::new(self.or_exp()?),
					);
				}
			}
			_ => {}
		}

		Ok(left)
	}

	fn or_exp(&self) -> Result<AST, String> {
		let mut left = self.and_exp()?;
		while self.is(0, Name::Or) {
			self.eat(Name::Or)?;
			left =
				AST::Op2(Name::Or, Box::new(left), Box::new(self.and_exp()?));
		}

		Ok(left)
	}

	fn and_exp(&self) -> Result<AST, String> {
		let mut left = self.equality_exp()?;
		while self.is(0, Name::And) {
			self.eat(Name::And)?;
			left = AST::Op2(
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
			left = AST::Op2(
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
			left = AST::Op2(
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
			left = AST::Op2(
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
			left = AST::Op2(
				t.of.name,
				Box::new(left),
				Box::new(self.exponential_exp()?),
			);
		}

		Ok(left)
	}

	fn exponential_exp(&self) -> Result<AST, String> {
		let mut left = self.range_exp()?;
		while self.is(0, Name::Exp) {
			self.eat(Name::Exp)?;
			left = AST::Op2(
				Name::Exp,
				Box::new(left),
				Box::new(self.range_exp()?),
			);
		}

		Ok(left)
	}

	fn range_exp(&self) -> Result<AST, String> {
		let mut left = self.unary_exp()?;
		while self.is(0, Name::Range) {
			self.eat(Name::Range)?;
			left = AST::Op2(
				Name::Range,
				Box::new(left),
				Box::new(self.unary_exp()?),
			);
		}

		Ok(left)
	}

	fn unary_exp(&self) -> Result<AST, String> {
		if self.any(
			0,
			&[
				Name::Add,
				Name::Sub,
				Name::Not,
				Name::Range,
				Name::Colon,
				Name::Gt,
				Name::Lt,
				Name::Length,
			],
		) {
			let operator = self.eats(&[
				Name::Add,
				Name::Sub,
				Name::Not,
				Name::Range,
				Name::Colon,
				Name::Gt,
				Name::Lt,
				Name::Length,
			])?;
			Ok(AST::Op1(operator.of.name, Box::new(self.unary_exp()?)))
		} else {
			self.replicate_or_select()
			// Ok(AST::Nothing)
			// Ok(self.literal()?)
		}
	}

	fn replicate_or_select(&self) -> Result<AST, String> {
		let mut ret = self.select_exp()?;
		if self.any(0, &[Name::ParenLF, Name::SquarenLF, Name::BracketLF]) {
			ret = AST::Rep(Box::new(ret), Box::new(self.expression()?));
		}
		Ok(ret)
		// }
	}

	fn select_exp(&self) -> Result<AST, String> {
		let mut left = self.primary_exp()?;
		while self.is(0, Name::Select) {
			let t = self.eat(Name::Select)?;
			left = AST::Op2(
				t.of.name,
				Box::new(left),
				Box::new(self.primary_exp()?),
			);
		}

		Ok(left)
	}

	fn primary_exp(&self) -> Result<AST, String> {
		match self.get(0) {
			Some(t) => match t.of.name {
				Name::ParenLF => self.paren_exp(),
				Name::SquarenLF => self.array_exp(),
				Name::BracketLF => self.graph_exp(),
				Name::Ref => self.reference(),
				_ => self.literal(),
			},
			None => self.literal(),
		}
	}

	fn reference(&self) -> Result<AST, String> {
		let t = self.eat(Name::Ref)?;
		Ok(AST::Ref(t.meta.text.clone()))
	}
	fn exp_list(&self, stops: &[Name]) -> Result<Vec<AST>, String> {
		let mut exps = vec![];

		self.clear_stops();
		while self.until(0, stops) {
			exps.push(self.expression()?);
			self.clear_stops();
		}

		Ok(exps)
	}

	// fn tuple_exp(&self) -> Result<AST, String> {
	// 	self.eat(Name::ParenLF)?;
	// 	let mut exps = self.exp_list(&[Name::ParenRT])?;
	// 	self.eat(Name::ParenRT)?;
	// 	if exps.len() == 1 {
	// 		Ok(exps.pop().unwrap())
	// 	} else {
	// 		Ok(AST::Tuple(exps))
	// 	}
	// }

	fn paren_exp(&self) -> Result<AST, String> {
		self.eat(Name::ParenLF)?;
		let expression = self.expression()?;
		self.eat(Name::ParenRT)?;
		Ok(expression)
	}

	fn array_exp(&self) -> Result<AST, String> {
		self.eat(Name::SquarenLF)?;
		let exps = self.exp_list(&[Name::SquarenRT])?;
		self.eat(Name::SquarenRT)?;
		Ok(AST::Array(exps))
	}

	fn graph_exp(&self) -> Result<AST, String> {
		self.eat(Name::BracketLF)?;
		let points = self.point_list(&[Name::BracketRT])?;
		self.eat(Name::BracketRT)?;
		Ok(AST::Graph(points))
	}

	fn literal(&self) -> Result<AST, String> {
		match self.get(0) {
			Some(t) => match t.of.kind {
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

	fn number(&self) -> Result<AST, String> {
		let t = self.eat_of(Kind::Number)?;
		Ok(AST::Number(name_to_number(t.of.name), t.meta.text.clone()))
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
				let ret = if self.any(0, names) {
					Ok(t)
				} else {
					Err(format!(
						"UnexpectedToken: {:?} on line {}\nExpected token of name: {:?}",
						t.meta.text, t.meta.line, t.of.name
					))
				};
				*self.cursor.borrow_mut() += 1; // must occur after self.any
				ret
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
