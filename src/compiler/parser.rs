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
	Rep(Box<AST>, Box<AST>), // replicate

	Binary(Name, Box<AST>, Box<AST>),
	Unary(Name, Box<AST>),
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

	fn point_list(&self, stops: &[Name]) -> Vec<AST> {
		let mut points = vec![];

		self.clear_stops();
		while self.until(0, stops) {
			points.push(self.point());
			self.clear_stops();
		}

		match points.last() {
			Some(AST::MisplacedExpression(exp)) => {
				let last = AST::Return(exp.clone());
				points.pop();
				points.push(last);
			}
			_ => {}
		}

		points
	}

	fn exp_list(&self, stops: &[Name]) -> Vec<AST> {
		let mut exps = vec![];

		self.clear_stops();
		while self.until(0, stops) {
			exps.push(self.expression());
			self.clear_stops();
		}

		exps
	}

	fn point(&self) -> AST {
		// assume a naked expresssion is an error, "disconnected point", if it's the last expression in graph, make it the return point

		self.clear_stops();
		if self.is(0, Name::Word) && self.is(1, Name::Colon) {
			let name = self.word_val();
			self.skip(); // eat ':'

			let point = AST::Point(name, Box::new(self.expression()));
			self.clear_stops();
			point
		} else {
			let expression = self.expression();
			self.clear_stops();
			AST::MisplacedExpression(Box::new(expression))
		}
	}
	fn word_val(&self) -> String {
		match self.eat_of(Class::Word) {
			Ok(t) => t.2.clone(),
			Err(e) => e,
		}
	}

	fn expression(&self) -> AST {
		self.or_exp()
	}

	fn pattern_exp(&self) -> AST {
		let mut left = self.or_exp();
		while self.is(0, Name::Pattern) {
			let _ = self.eat_of(Class::Binary);
			left = AST::Binary(
				Name::Pattern,
				Box::new(left),
				Box::new(self.or_exp()),
			);
		}

		left
	}

	fn or_exp(&self) -> AST {
		let mut left = self.and_exp();
		while self.is(0, Name::Or) {
			let _ = self.eat_of(Class::Binary);
			left = AST::Binary(
				Name::Or,
				Box::new(left),
				Box::new(self.and_exp()),
			);
		}

		left
	}

	fn and_exp(&self) -> AST {
		let mut left = self.equality_exp();
		while self.is(0, Name::And) {
			let _ = self.eat_of(Class::Binary);
			left = AST::Binary(
				Name::And,
				Box::new(left),
				Box::new(self.equality_exp()),
			);
		}

		left
	}

	fn equality_exp(&self) -> AST {
		let mut left = self.relation_exp();
		while self.any(0, &[Name::Eq, Name::Ne]) {
			let t = self.eat_of(Class::Binary).unwrap();
			left = AST::Binary(
				t.1,
				Box::new(left),
				Box::new(self.relation_exp()),
			);
		}

		left
	}

	fn relation_exp(&self) -> AST {
		let mut left = self.additive_exp();
		while self.any(0, &[Name::Gt, Name::Ge, Name::Lt, Name::Le]) {
			let t = self.eat_of(Class::Binary).unwrap();
			left = AST::Binary(
				t.1,
				Box::new(left),
				Box::new(self.additive_exp()),
			);
		}

		left
	}

	fn additive_exp(&self) -> AST {
		let mut left = self.multiplicative_exp();
		while self.any(0, &[Name::Add, Name::Sub]) {
			let t = self.eat_of(Class::Binary).unwrap();
			left = AST::Binary(
				t.1,
				Box::new(left),
				Box::new(self.multiplicative_exp()),
			);
		}

		left
	}

	fn multiplicative_exp(&self) -> AST {
		let mut left = self.exponential_exp();
		while self.any(0, &[Name::Mul, Name::Div]) {
			let t = self.eat_of(Class::Binary).unwrap();
			left = AST::Binary(
				t.1,
				Box::new(left),
				Box::new(self.exponential_exp()),
			);
		}

		left
	}

	fn exponential_exp(&self) -> AST {
		let mut left = self.unary_exp();
		while self.any(0, &[Name::Exp]) {
			let t = self.eat_of(Class::Binary).unwrap();
			left =
				AST::Binary(t.1, Box::new(left), Box::new(self.unary_exp()));
		}

		left
	}

	fn unary_exp(&self) -> AST {
		if self.any(0, &[Name::Add, Name::Sub, Name::Not]) {
			let operator = self.eat_any();
			AST::Unary(operator.1, Box::new(self.unary_exp()))
		} else {
			self.replicate_or_select()
		}
	}

	fn replicate_or_select(&self) -> AST {
		let mut ret = self.select_exp();
		if self.any(0, &[Name::ParenLF, Name::SquarenLF, Name::BracketLF]) {
			let arg = self.replicate_arg();
			ret = AST::Rep(Box::new(ret), Box::new(arg));
		}
		ret
		// }
	}

	fn replicate_arg(&self) -> AST {
		self.expression()
	}

	fn select_exp(&self) -> AST {
		let mut left = self.primary_exp();
		while self.of(0, Class::Select) {
			let t = self.eat_of(Class::Select).unwrap();
			left =
				AST::Binary(t.1, Box::new(left), Box::new(self.selector()));
		}

		left
	}
	fn selector(&self) -> AST {
		self.skip()
	}
	fn primary_exp(&self) -> AST {
		// self.skip()
		match self.get(0) {
			Some((_, Name::ParenLF, _)) => self.tuple_exp(),
			Some((_, Name::SquarenLF, _)) => self.array_exp(),
			Some((_, Name::BracketLF, _)) => self.graph_exp(),
			Some((_, Name::Word, _)) => self.reference(),
			Some(_) => self.literal(),
			None => self.literal(), // will return error
		}
	}

	fn reference(&self) -> AST {
		match self.eat_of(Class::Word) {
			Ok(t) => AST::Ref(t.2.clone()),
			Err(e) => AST::Error(e),
		}
	}

	fn tuple_exp(&self) -> AST {
		// self.skip()
		let _ = self.eat(Name::ParenLF);
		let mut exps = self.exp_list(&[Name::ParenRT]);
		let _ = self.eat(Name::ParenRT);
		if exps.len() == 1 {
			exps.pop().unwrap()
		} else {
			AST::Tuple(exps)
		}
	}

	fn array_exp(&self) -> AST {
		let _ = self.eat(Name::SquarenLF);
		let exps = self.exp_list(&[Name::SquarenRT]);
		let _ = self.eat(Name::SquarenRT);
		AST::Array(exps)
	}

	fn graph_exp(&self) -> AST {
		let _ = self.eat(Name::BracketLF);
		let points = self.point_list(&[Name::BracketRT]);
		let _ = self.eat(Name::BracketRT);
		AST::Graph(points)
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
		match self.eat_of(Class::Bool) {
			Ok(t) => AST::Bool(if t.1 == Name::True { true } else { false }),
			Err(e) => AST::Error(e),
		}
	}

	fn number(&self) -> AST {
		match self.eat_of(Class::Number) {
			Ok(t) => AST::Number(t.2.clone()),
			Err(e) => AST::Error(e),
		}
	}

	fn string(&self) -> AST {
		match self.eat_of(Class::String) {
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
	fn eat(&self, name: Name) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if t.1 == name {
					Ok(t)
				} else {
					Err(format!("UnexpectedToken: {}", t.2))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}
	fn eat_of(&self, class: Class) -> Result<&Token, String> {
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
	fn skip(&self) -> AST {
		*self.cursor.borrow_mut() += 1;
		AST::Nothing
	}

	fn eat_any(&self) -> &Token {
		self.next().unwrap()
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

	fn any(&self, offset: usize, stops: &[Name]) -> bool {
		for stop in stops {
			if self.is(offset, *stop) {
				return true;
			}
		}
		false
	}

	fn any_of(&self, offset: usize, stops: &[Class]) -> bool {
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

	fn until(&self, offset: usize, stops: &[Name]) -> bool {
		match self.get(offset) {
			Some((_, name, _)) => {
				for stop in stops {
					if *name == *stop {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
	fn until_class(&self, offset: usize, stops: &[Class]) -> bool {
		match self.get(offset) {
			Some((class, _, _)) => {
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
