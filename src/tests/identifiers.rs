use crate::parser::{Expr, Parser};

#[test]
fn test_identifier_read() {
    match Parser::new("foo").parse_expr().unwrap() {
        Expr::Read(ref s) if s == "foo" => {}
        _ => panic!("Expected identifier read"),
    }
}
