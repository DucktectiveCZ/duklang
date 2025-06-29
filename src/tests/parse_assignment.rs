use crate::parser::{Expr, LiteralExpr, Parser};

#[test]
fn test_string_literal() {
    match Parser::new("= 10").parse_assignment().unwrap() {
        Some(Expr::Literal(LiteralExpr::Int(10))) => {}
        _ => panic!("Expected string literal"),
    }
}
