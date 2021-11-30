use rocklang::token::Token;

#[test]
fn it_returns_not_equal_when_formatng() {
    let token = Token::NotEqual;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "NotEqual");
}

#[test]
fn it_returns_left_paren_when_formating() {
    let token = Token::LeftParen;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "LeftParen");
}

#[test]
fn it_returns_right_paren_when_formating() {
    let token = Token::RightParen;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "RightParen");
}

#[test]
fn it_returns_slash_when_formating() {
    let token = Token::Slash;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Slash");
}

#[test]
fn it_returns_identifier_when_formating() {
    let token = Token::Identifier("ident".to_string());

    let token_name = format!("{}", token);
    assert_eq!(token_name, "<Identifier(ident)>");
}

#[test]
fn it_returns_numeric_when_formating() {
    let token = Token::Numeric(10.0);

    let token_name = format!("{}", token);
    assert_eq!(token_name, "<Numeric(10)>");
}

#[test]
fn it_returns_plus_when_formating() {
    let token = Token::Plus;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Plus");
}

#[test]
fn it_returns_minus_when_formating() {
    let token = Token::Minus;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Minus");
}

#[test]
fn it_returns_asterisk_when_formating() {
    let token = Token::Asterisk;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Asterisk");
}

#[test]
fn it_returns_equal_when_formating() {
    let token = Token::Equal;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Equal");
}

#[test]
fn it_returns_l_curly_when_formating() {
    let token = Token::LCurly;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "LCurly");
}

#[test]
fn it_returns_r_curly_when_formating() {
    let token = Token::RCurly;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "RCurly");
}

#[test]
fn it_returns_if_when_formating() {
    let token = Token::If;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "If");
}

#[test]
fn it_returns_while_when_formating() {
    let token = Token::While;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "While");
}

#[test]
fn it_returns_true_when_formating() {
    let token = Token::True;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "True");
}

#[test]
fn it_returns_false_when_formating() {
    let token = Token::False;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "False");
}

#[test]
fn it_returns_double_equal_when_formating() {
    let token = Token::DoubleEqual;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "DoubleEqual");
}

#[test]
fn it_returns_percent_when_formating() {
    let token = Token::Percent;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Percent");
}

#[test]
fn it_returns_exclamation_when_formating() {
    let token = Token::Exclamation;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Exclamation");
}

#[test]
fn it_returns_break_when_formating() {
    let token = Token::Break;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Break");
}

#[test]
fn it_returns_string_when_formating() {
    let token = Token::String("string".to_string());

    let token_name = format!("{}", token);
    assert_eq!(token_name, "<String>");
}

#[test]
fn it_returns_eof_when_formating() {
    let token = Token::Eof;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Eof");
}

#[test]
fn it_returns_comma_when_formating() {
    let token = Token::Comma;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Comma");
}

#[test]
fn it_returns_arrow_when_formating() {
    let token = Token::Arrow;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Arrow");
}

#[test]
fn it_returns_less_when_formating() {
    let token = Token::Less;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Less");
}

#[test]
fn it_returns_less_or_equal_when_formating() {
    let token = Token::LessOrEqual;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "LessOrEqual");
}

#[test]
fn it_returns_greater_when_formating() {
    let token = Token::Greater;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Greater");
}

#[test]
fn it_returns_greater_or_equal_when_formating() {
    let token = Token::GreaterOrEqual;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "GreaterOrEqual");
}

#[test]
fn it_returns_or_when_formating() {
    let token = Token::Or;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Or");
}

#[test]
fn it_returns_and_when_formating() {
    let token = Token::And;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "And");
}

#[test]
fn it_returns_else_when_formating() {
    let token = Token::Else;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Else");
}

#[test]
fn it_returns_colon_when_formating() {
    let token = Token::Colon;

    let token_name = format!("{:?}", token);
    assert_eq!(token_name, "Colon");
}
