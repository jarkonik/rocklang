use assert_json_diff::assert_json_eq;
use rocklang::parser::{Parse, Parser};
use rocklang::token::Token;
use serde_json::json;

#[test]
fn it_parses_addition() {
	let mut parser = Parser::new(&vec![
		Token::Numeric(5.2),
		Token::Plus,
		Token::Numeric(10.0),
		Token::Eof,
	]);

	let ast = parser.parse().unwrap().body;
	let json = serde_json::to_value(&ast).unwrap();

	assert_json_eq!(
		json!(
			[
				{
					"Binary": {
						"left": {
							"Numeric":5.2
						},
						"operator":"Plus",
						"right": {
							"Numeric":10.0
						}
					}
				}
			]
		),
		json
	)
}
