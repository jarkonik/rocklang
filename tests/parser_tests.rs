use assert_json_diff::assert_json_eq;
use rocklang::parser::SyntaxError;
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

#[test]
fn it_parses_parentheses() {
    let mut parser = Parser::new(&vec![
        Token::LeftParen,
        Token::Numeric(10.0),
        Token::Plus,
        Token::Numeric(2.0),
        Token::RightParen,
        Token::Slash,
        Token::Numeric(3.0),
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
                            "Grouping": {
                                "Binary": {
                                    "left":{
                                        "Numeric": 10.0
                                    },
                                    "operator": "Plus",
                                    "right":{
                                        "Numeric": 2.0
                                    }
                                }
                            }
                        },
                        "operator":"Slash",
                        "right": {
                            "Numeric": 3.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn parses_while_loop() {
    let mut parser = Parser::new(&vec![
        Token::While,
        Token::Identifier("x".to_string()),
        Token::Less,
        Token::Numeric(10.0),
        Token::LCurly,
        Token::Identifier("print".to_string()),
        Token::LeftParen,
        Token::String("hello".to_string()),
        Token::RightParen,
        Token::RCurly,
        Token::Eof,
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "While": {
                        "predicate": {
                            "Binary": {
                                "left":{
                                    "Identifier": "x"
                                },
                                "operator": "Less",
                                "right":{
                                    "Numeric": 10.0
                                }
                            }
                        },
                        "body": [
                            {
                                "FuncCall": {
                                    "args": [
                                        {
                                        "String": "hello"
                                        }
                                    ],
                                    "calee": {
                                        "Identifier": "print"
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn return_error_when_no_curly_after_while_predicate() {
    let mut parser = Parser::new(&vec![
        Token::While,
        Token::Identifier("x".to_string()),
        Token::Less,
        Token::Numeric(10.0),
        Token::String("hello".to_string()),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert_eq!(
                SyntaxError {
                    token: Token::String("hello".to_string())
                },
                e
            );
        }
    };
}

#[test]
fn syntax_error_display() {
    let error = SyntaxError {
        token: Token::DoubleEqual,
    };
    assert_eq!(
        format!("Syntax error: unexpected token {}", Token::DoubleEqual),
        format!("{}", error)
    );
}
