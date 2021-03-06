use rocklang::token::Token;
use rocklang::tokenizer::TokenizerError;
use rocklang::tokenizer::{Tokenize, Tokenizer};

#[test]
fn it_tokenizes_all_tokens() {
    let mut tokenizer = Tokenizer::new(String::from(
        "< > () + - * % \"test\" !  != || && = == => { } ,
        // comment
         / identifier identifier123 23123.321 123 <= >= :",
    ));
    let tokens = tokenizer.tokenize().unwrap();
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
    assert_eq!(Token::LessOrEqual, tokens[24]);
    assert_eq!(Token::GreaterOrEqual, tokens[25]);
    assert_eq!(Token::Colon, tokens[26]);
    assert_eq!(Token::Eof, tokens[27]);
    assert_eq!(28, tokens.len());
}

#[test]
fn it_returns_error_for_unexpected_character() {
    let mut tokenizer = Tokenizer::new(String::from("~"));
    assert_eq!(
        Err(TokenizerError { chr: '~', line: 1 }),
        tokenizer.tokenize()
    );
}

#[test]
fn tokenizer_error_display() {
    let error = TokenizerError { chr: 'a', line: 55 };
    assert_eq!("unexpected character 'a'", format!("{}", error));
}
