use assert_json_diff::assert_json_eq;
use backtrace::Backtrace;
use rocklang::parser::ParserError;
use rocklang::parser::{Parse, Parser, Span};
use rocklang::token::{Token, TokenKind};
use serde_json::json;

macro_rules! token {
    ($kind:expr) => {
        Token {
            kind: $kind,
            span: Span::default(),
        }
    };
}

#[test]
fn it_parses_addition() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Numeric(5.2)),
        token!(TokenKind::Plus),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
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
        token!(TokenKind::LeftParen),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Plus),
        token!(TokenKind::Numeric(2.0)),
        token!(TokenKind::RightParen),
        token!(TokenKind::Slash),
        token!(TokenKind::Numeric(3.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "span": {
                        "column": 0,
                        "line": 0
                    },
                    "expression": {
                        "Binary": {
                            "left": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Grouping": {
                                        "span": {
                                            "column": 0,
                                            "line": 0
                                        },
                                        "expression": {
                                            "Binary": {
                                                "left":{
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                    "expression": {
                                                        "Numeric": 10.0
                                                    }
                                                },
                                                "operator": "Plus",
                                                "right":{
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                    "expression": {
                                                        "Numeric": 2.0
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "operator":"Slash",
                            "right": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Numeric": 3.0
                                }
                            }
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_while_loop() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::While),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "span": {
                        "column": 0,
                        "line": 0
                    },
                    "expression": {
                        "While": {
                            "predicate": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Binary": {
                                        "left":{
                                            "span": {
                                                "column": 0,
                                                "line": 0
                                            },
                                            "expression": {
                                                "Identifier": "x"
                                            }
                                        },
                                        "operator": "Less",
                                        "right":{
                                            "span": {
                                                "column": 0,
                                                "line": 0
                                            },
                                            "expression": {
                                                "Numeric": 10.0
                                            }
                                        }
                                    }
                                }
                            },
                            "body": [
                                {
                                    "span": {
                                        "column": 0,
                                        "line": 0
                                    },
                                    "expression": {
                                        "FuncCall": {
                                            "args": [
                                                {
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                    "expression": {
                                                        "String": "hello"
                                                    }
                                                }
                                            ],
                                            "calee": {
                                                "span": {
                                                    "column": 0,
                                                    "line": 0
                                                },
                                                "expression": {
                                                    "Identifier": "print"
                                                }
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_returns_error_when_no_curly_after_while_predicate_in_while() {
    let mut parser = Parser::new(&[
        token!(TokenKind::While),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::String("hello".to_string())),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                }
            ));
        }
    };
}

#[test]
fn it_parses_conditionals() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::If),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Conditional": {
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
                        ],
                        "else_body": []
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_conditionals_with_else() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::If),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Else),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("else".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Conditional": {
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
                        ],
                        "else_body": [
                            {
                                "FuncCall": {
                                    "args": [
                                        {
                                        "String": "else"
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
fn it_returns_error_when_no_curly_after_while_predicate_in_if() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::If),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Else),
        token!(TokenKind::String("hello".to_string())),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_error_when_no_curly_after_while_predicate_in_else() {
    let mut parser = Parser::new(&[
        token!(TokenKind::If),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::String("hello".to_string())),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_displays_correct_syntax_error() {
    let error = ParserError::SyntaxError {
        token: token!(TokenKind::DoubleEqual),
        backtrace: Backtrace::new(),
    };
    assert_eq!(
        format!(
            "Syntax error: unexpected token {} at 0:0",
            TokenKind::DoubleEqual
        ),
        format!("{}", error)
    );
}

#[test]
fn it_parses_assignments() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Equal),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Assignment": {
                        "left": {
                            "Identifier": "x"
                        },
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_binary_equal() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::DoubleEqual),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "Equal",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_binary_not_equal() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::NotEqual),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "NotEqual",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_less_or_equal() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::LessOrEqual),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "LessOrEqual",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_less() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Less),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "Less",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_greater() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Greater),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "Greater",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_greater_or_equal() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::GreaterOrEqual),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Identifier": "x"
                        },
                        "operator": "GreaterOrEqual",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_subtraction() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Minus),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "span": {
                        "column": 0,
                        "line": 0
                    },
                    "expression": {
                        "Binary": {
                            "left": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Numeric": 10.0
                                }
                            },
                            "operator": "Minus",
                            "right": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Identifier": "x"
                                }
                            }
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_modulo() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Percent),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Numeric": 10.0
                        },
                        "operator": "Mod",
                        "right": {
                            "Identifier": "x"
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_multiplication() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Asterisk),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Numeric": 10.0
                        },
                        "operator": "Asterisk",
                        "right": {
                            "Identifier": "x"
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_division() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Slash),
        token!(TokenKind::Identifier("x".to_string())),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Binary": {
                        "left": {
                            "Numeric": 10.0
                        },
                        "operator": "Slash",
                        "right": {
                            "Identifier": "x"
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_unary_minus() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Minus),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Unary": {
                        "operator": "Minus",
                        "right": {
                            "Numeric": 10.0
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_func_declaration_with_no_params() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncDecl": {
                        "return_type": "Null",
                        "params": [],
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
fn it_parses_func_declaration_with_one_number_param() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("number".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncDecl": {
                        "return_type": "Null",
                        "params": [
                            {
                                "typ": "Numeric",
                                "name": "a"
                            }
                        ],
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
fn it_parses_func_declaration_with_one_vec_param() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("vec".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncDecl": {
                        "return_type": "Null",
                        "params": [
                            {
                                "typ": "Vector",
                                "name": "a"
                            }
                        ],
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
fn it_parses_func_declaration_with_one_fun_param() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "span": {
                        "column": 0,
                        "line": 0
                    },
                    "expression": {
                        "FuncDecl": {
                            "return_type": "Void",
                            "params": [
                                {
                                    "typ": "Function",
                                    "name": "a"
                                }
                            ],
                            "body": [
                                {
                                    "span": {
                                        "column": 0,
                                        "line": 0
                                    },
                                    "expression": {
                                        "FuncCall": {
                                            "args": [
                                                {
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                    "expression": {

                                                        "String": "hello"
                                                    }
                                                }
                                            ],
                                            "calee": {
                                                "span": {
                                                    "column": 0,
                                                    "line": 0
                                                },
                                                "expression": {
                                                    "Identifier": "print"
                                                }
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_func_declaration_with_multiple_params() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Identifier("b".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("number".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncDecl": {
                        "return_type": "Null",
                        "params": [
                            {
                                "typ": "Function",
                                "name": "a"
                            },
                            {
                                "typ": "Numeric",
                                "name": "b"
                            }
                        ],
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
fn it_returns_an_error_when_func_has_unknown_arg_type() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Identifier("b".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("wrongtype".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::Identifier(_),
                        ..
                    },
                    ..
                }
            ));
        }
    };
}

#[test]
fn it_returns_an_error_when_func_has_non_type_expression_as_arg_type() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Identifier("b".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::String("string".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_an_error_when_func_has_no_arg_type_after_arg_name() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Identifier("b".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::Colon,
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_an_error_when_func_has_no_return_type() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::Arrow,
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_an_error_when_func_has_unknown_return_type() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("wrongtype".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::Identifier(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_an_error_when_func_has_non_type_expression_as_return_type() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::String("string".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_error_when_func_decl_has_no_arrow() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::LCurly),
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::RCurly),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::LCurly,
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_returns_error_when_func_decl_has_no_body() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("fun".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Colon),
        token!(TokenKind::Identifier("void".to_string())),
        token!(TokenKind::Arrow),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_parses_func_call() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncCall": {
                        "args": [],
                        "calee": {
                            "Identifier": "print"
                        }

                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_func_call_with_one_arg() {
    let mut parser = Parser::new(&[
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
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
        ),
        json
    )
}

#[test]
fn it_parses_func_call_with_two_args() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::Identifier("print".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "FuncCall": {
                        "args": [
                            {
                                "String": "hello"
                            },
                            {
                                "Numeric": 10.0
                            }
                        ],
                        "calee": {
                            "Identifier": "print"
                        }

                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_returns_error_for_call_syntax_on_non_identifiers() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::Comma),
        token!(TokenKind::Numeric(10.0)),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::LeftParen,
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_parses_grouping_expression() {
    let mut parser = Parser::new(&[
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Grouping": {
                        "String": "hello"
                    }
                }
            ]
        ),
        json
    )
}

#[test]
fn it_returns_error_for_unterminated_grouping_expresions() {
    let mut parser = Parser::new(&[
        token!(TokenKind::LeftParen),
        token!(TokenKind::String("hello".to_string())),
        token!(TokenKind::Eof),
    ]);

    match parser.parse() {
        Ok(_) => assert!(false, "should return an error"),
        Err(e) => {
            assert!(matches!(
                e,
                ParserError::SyntaxError {
                    token: Token {
                        kind: TokenKind::String(_),
                        ..
                    },
                    ..
                },
            ));
        }
    };
}

#[test]
fn it_parses_true_bool_literal() {
    let mut parser = Parser::new(&[token!(TokenKind::True), token!(TokenKind::Eof)]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Bool": true
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_false_bool_literal() {
    let mut parser = Parser::new(&[token!(TokenKind::False), token!(TokenKind::Eof)]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!(
            [
                {
                    "Bool": false
                }
            ]
        ),
        json
    )
}

#[test]
fn it_parses_break_expression() {
    let mut parser = Parser::new(&[token!(TokenKind::Break), token!(TokenKind::Eof)]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!([{ "expression": "Break", "span": {
        "column": 0,
        "line": 0
    } }]),
        json
    )
}

#[test]
fn it_parses_grouping_expression_with_identifiers() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::Identifier("b".to_string())),
        token!(TokenKind::Equal),
        token!(TokenKind::LeftParen),
        token!(TokenKind::Identifier("a".to_string())),
        token!(TokenKind::Plus),
        token!(TokenKind::Numeric(1.0)),
        token!(TokenKind::RightParen),
        token!(TokenKind::Asterisk),
        token!(TokenKind::Numeric(2.0)),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!([{
            "span": {
                "column": 0,
                "line": 0
            },
            "expression": {
                "Assignment": {
                    "left": {
                        "expression": {
                            "Identifier": "b"
                        },
                        "span": {
                            "column": 0,
                            "line": 0
                        },
                    },
                "right": {
                    "span": {
                        "column": 0,
                        "line": 0
                    },
                    "expression": {
                        "Binary": {
                            "left": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Grouping": {
                                        "span": {
                                            "column": 0,
                                            "line": 0
                                        },
                                        "expression": {
                                            "Binary": {
                                                "left": {
                                                    "expression": {
                                                        "Identifier": "a"
                                                    },
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                },
                                                "operator": "Plus",
                                                "right": {
                                                    "expression": {
                                                        "Numeric": 1.0,
                                                    },
                                                    "span": {
                                                        "column": 0,
                                                        "line": 0
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "operator": "Asterisk",
                            "right": {
                                "span": {
                                    "column": 0,
                                    "line": 0
                                },
                                "expression": {
                                    "Numeric": 2.0
                                }
                            }
                        }
                    }
                }
            }
            }
        }]),
        json
    )
}

#[test]
fn it_parses_load_expression() {
    let mut parser = Parser::new(&vec![
        token!(TokenKind::Load),
        token!(TokenKind::LeftParen),
        token!(TokenKind::String(String::from("somelib.so"))),
        token!(TokenKind::RightParen),
        token!(TokenKind::Eof),
    ]);

    let ast = parser.parse().unwrap().body;
    let json = serde_json::to_value(&ast).unwrap();

    assert_json_eq!(
        json!([{
            "span": {
                "column": 0,
                "line": 0
            },
            "expression": {
                "Load": "somelib.so"
            }
        }]),
        json
    )
}
