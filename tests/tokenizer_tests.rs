use rocklang::token::TokenKind;
use rocklang::tokenizer::TokenizerError;
use rocklang::tokenizer::{Tokenize, Tokenizer};

macro_rules! assert_token_kind_of {
    ($expected: expr, $actual: expr) => {
        assert_eq!($expected, $actual.kind);
    };
}

#[test]
fn it_tokenizes_all_tokens() {
    let mut tokenizer = Tokenizer::new(String::from(
        "< > () + - * % \"test\" !  != || && = == => { } ,
        // comment
         / identifier identifier123 23123.321 123 <= >= :",
    ));
    let tokens = tokenizer.tokenize().unwrap();
    assert_token_kind_of!(TokenKind::Less, tokens[0]);
    assert_token_kind_of!(TokenKind::Greater, tokens[1]);
    assert_token_kind_of!(TokenKind::LeftParen, tokens[2]);
    assert_token_kind_of!(TokenKind::RightParen, tokens[3]);
    assert_token_kind_of!(TokenKind::Plus, tokens[4]);
    assert_token_kind_of!(TokenKind::Minus, tokens[5]);
    assert_token_kind_of!(TokenKind::Asterisk, tokens[6]);
    assert_token_kind_of!(TokenKind::Percent, tokens[7]);
    assert_token_kind_of!(TokenKind::String(String::from("test")), tokens[8]);
    assert_token_kind_of!(TokenKind::Exclamation, tokens[9]);
    assert_token_kind_of!(TokenKind::NotEqual, tokens[10]);
    assert_token_kind_of!(TokenKind::Or, tokens[11]);
    assert_token_kind_of!(TokenKind::And, tokens[12]);
    assert_token_kind_of!(TokenKind::Equal, tokens[13]);
    assert_token_kind_of!(TokenKind::DoubleEqual, tokens[14]);
    assert_token_kind_of!(TokenKind::Arrow, tokens[15]);
    assert_token_kind_of!(TokenKind::LCurly, tokens[16]);
    assert_token_kind_of!(TokenKind::RCurly, tokens[17]);
    assert_token_kind_of!(TokenKind::Comma, tokens[18]);
    assert_token_kind_of!(TokenKind::Slash, tokens[19]);
    assert_token_kind_of!(
        TokenKind::Identifier(String::from("identifier")),
        tokens[20]
    );
    assert_token_kind_of!(
        TokenKind::Identifier(String::from("identifier123")),
        tokens[21]
    );
    assert_token_kind_of!(TokenKind::Numeric(23123.321), tokens[22]);
    assert_token_kind_of!(TokenKind::Numeric(123.0), tokens[23]);
    assert_token_kind_of!(TokenKind::LessOrEqual, tokens[24]);
    assert_token_kind_of!(TokenKind::GreaterOrEqual, tokens[25]);
    assert_token_kind_of!(TokenKind::Colon, tokens[26]);
    assert_token_kind_of!(TokenKind::Eof, tokens[27]);
    assert_eq!(28, tokens.len());
}

#[test]
fn it_returns_error_for_unexpected_character() {
    let mut tokenizer = Tokenizer::new(String::from("~"));
    assert!(matches!(
        tokenizer.tokenize(),
        Err(TokenizerError { chr: '~', line: 1 }),
    ));
}

#[test]
fn tokenizer_error_display() {
    let error = TokenizerError { chr: 'a', line: 55 };
    assert_eq!("unexpected character 'a'", format!("{}", error));
}
