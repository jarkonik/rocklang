use rocklang::token::TokenKind;

macro_rules! assert_format_string {
    ($token:ident, $results:expr) => {
        let token = TokenKind::$token;

        let token_name = format!("{}", token);
        assert_eq!(token_name, $results);
    };
}

macro_rules! assert_angle_brackets_format_string {
    ($token:ident, $first_param:expr, $results:expr ) => {
        let token = TokenKind::$token($first_param);

        let token_name = format!("{}", token);
        assert_eq!(token_name, $results);
    };
}

#[test]
fn it_assert_token_types_formatng() {
    assert_angle_brackets_format_string!(Identifier, "ident".to_string(), "<Identifier(ident)>");
    assert_angle_brackets_format_string!(F64, 10.0, "<F64(10)>");
    assert_angle_brackets_format_string!(String, "string".to_string(), "<String(string)>");
    assert_format_string!(LeftParen, "<LeftParen>");
    assert_format_string!(NotEqual, "<NotEqual>");
    assert_format_string!(LeftParen, "<LeftParen>");
    assert_format_string!(RightParen, "<RightParen>");
    assert_format_string!(Slash, "<Slash>");
    assert_format_string!(Plus, "<Plus>");
    assert_format_string!(Minus, "<Minus>");
    assert_format_string!(Asterisk, "<Asterisk>");
    assert_format_string!(Equal, "<Equal>");
    assert_format_string!(LCurly, "<LCurly>");
    assert_format_string!(RCurly, "<RCurly>");
    assert_format_string!(If, "<If>");
    assert_format_string!(While, "<While>");
    assert_format_string!(True, "<True>");
    assert_format_string!(False, "<False>");
    assert_format_string!(DoubleEqual, "<DoubleEqual>");
    assert_format_string!(Percent, "<Percent>");
    assert_format_string!(Exclamation, "<Exclamation>");
    assert_format_string!(Break, "<Break>");
    assert_format_string!(Eof, "<Eof>");
    assert_format_string!(Comma, "<Comma>");
    assert_format_string!(Arrow, "<Arrow>");
    assert_format_string!(Less, "<Less>");
    assert_format_string!(LessOrEqual, "<LessOrEqual>");
    assert_format_string!(Greater, "<Greater>");
    assert_format_string!(GreaterOrEqual, "<GreaterOrEqual>");
    assert_format_string!(Or, "<Or>");
    assert_format_string!(And, "<And>");
    assert_format_string!(Else, "<Else>");
    assert_format_string!(Colon, "<Colon>");
}
