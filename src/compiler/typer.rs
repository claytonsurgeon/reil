/*
namespace should be kept out of ast
namespace: {
	program: {
		parent: Nothing
		children: {
			a: {

			}
			b: {

			}
			c: {

			}
		}
	}

}


{	label: "Program"
	type graph(Nothing)
	namespace: link
	data: [
		length: 3
		{	label: "a"
			type: i32
			namespace: link
			data: 10
		}
		{	label: "b"
			type: i32
			namespace: link
			data: 20
		}
		{	label: "c"
			type: Op2(+, i32)
			namespace: link
			data: [
				length: 2
				{	label: ""
					type: ref(i32)
					data: ref(a)
				}
				{	label: ""
					type: ref(i32)
					data: ref(b)
				}
			]
		}
	]
}

*/
use super::parser::{Number, AST};
use super::tokenizer::Name;
use std::collections::HashMap;

// enum Data {
// 	V(Vec<Node>),
// 	B(Box<Node>),
// }

// pub struct Node {
// 	label: String,
// 	position: i32,
// 	kind: Primitive,
// 	namespace: HashMap<String, Node>,
// 	data: Data,
// }

pub fn typer(ast: &AST) -> Result<AST, String> {
	// let mut namespace: HashMap<String, Primitive> = HashMap::new();

	// // Review some books.
	// book_reviews
	// 	.insert("Adventures of Huckleberry Finn".to_string(), Primitive::Nothing);
	Err("nothing to see".to_string())
}
