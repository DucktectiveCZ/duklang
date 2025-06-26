use crate::parser::{ParseError, Parser};

#[test]
fn test_error_expected_token() {
    let mut parser = Parser::new("");
    assert!(matches!(parser.parse_expr().unwrap_err(), ParseError::ExpectedToken));
}

#[test]
fn test_error_unexpected_token() {
    let mut parser = Parser::new(";");
    assert!(matches!(parser.parse_expr().unwrap_err(), ParseError::UnexpectedToken(_)));
}

#[test]
fn test_error_missing_closing_paren() {
    let mut parser = Parser::new("foo(1, 2");
    assert!(matches!(parser.parse_expr().unwrap_err(), ParseError::ExpectedToken));
}
