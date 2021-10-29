use rocklang::token::Token;
use rocklang::tokenizer::{Tokenize, Tokenizer};

#[test]
fn it_tokenizes_all_tokens() {
    let mut tokenizer = Tokenizer::new(String::from(
		"< > () + - * % \"test\" !  != || && = == => { } , / identifier identifier123 23123.321 123",
	));
    let tokens = tokenizer.tokenize();
    assert_eq!(25, tokens.len());
    assert_eq!(Token::Less, tokens[0]);
    assert_eq!(Token::Greater, tokens[1]);
    assert_eq!(Token::LeftParen, tokens[2]);
    assert_eq!(Token::RightParen, tokens[3]);
    assert_eq!(Token::Plus, tokens[4]);
    assert_eq!(Token::Minus, tokens[5]);
    assert_eq!(Token::Asterisk, tokens[6]);
    assert_eq!(Token::Percent, tokens[7]);
    assert_eq!(Token::String(String::from("test")), tokens[8]);
    assert_eq!(Token::Exclamation, tokens[9]);
    assert_eq!(Token::NotEqual, tokens[10]);
    assert_eq!(Token::Or, tokens[11]);
    assert_eq!(Token::And, tokens[12]);
    assert_eq!(Token::Equal, tokens[13]);
    assert_eq!(Token::DoubleEqual, tokens[14]);
    assert_eq!(Token::Arrow, tokens[15]);
    assert_eq!(Token::LCurly, tokens[16]);
    assert_eq!(Token::RCurly, tokens[17]);
    assert_eq!(Token::Comma, tokens[18]);
    assert_eq!(Token::Slash, tokens[19]);
    assert_eq!(Token::Identifier(String::from("identifier")), tokens[20]);
    assert_eq!(Token::Identifier(String::from("identifier123")), tokens[21]);
    assert_eq!(Token::Numeric(23123.321), tokens[22]);
    assert_eq!(Token::Numeric(123.0), tokens[23]);
    assert_eq!(Token::Eof, tokens[24]);
}
