use crate::parser::{Expr, LiteralExpr, Parser};

#[test]
fn test_int_literal() {
    match Parser::new("42").parse_expr().unwrap() {
        Expr::Literal(LiteralExpr::Int(42)) => {}
        _ => panic!("Expected int literal"),
    }
}

#[test]
fn test_uint_literal() {
    match Parser::new("42u").parse_expr().unwrap() {
        Expr::Literal(LiteralExpr::UInt(42)) => {}
        x => panic!("Expected uint literal, found {:?}", x),
    }
}

#[test]
fn test_float_literal() {
    match Parser::new("3.14").parse_expr().unwrap() {
        Expr::Literal(LiteralExpr::Float(val)) if (val - 3.14).abs() < 1e-8 => {}
        _ => panic!("Expected float literal"),
    }
}

#[test]
fn test_string_literal() {
    match Parser::new("\"hello\"").parse_expr().unwrap() {
        Expr::Literal(LiteralExpr::Str(ref s)) if s == "hello" => {}
        _ => panic!("Expected string literal"),
    }
}
