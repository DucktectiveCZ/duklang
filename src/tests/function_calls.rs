use crate::parser::{Expr, Parser};

#[test]
fn test_function_call_no_args() {
    match Parser::new("foo()").parse_expr().unwrap() {
        Expr::Call {
            ref callee,
            ref args,
        } if callee == "foo" && args.is_empty() => {}
        _ => panic!("Expected function call with no args"),
    }
}

#[test]
fn test_function_call_with_args() {
    match Parser::new("foo(1, 2,)").parse_expr().unwrap() {
        Expr::Call {
            ref callee,
            ref args,
        } if callee == "foo" => {
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected function call with args"),
    }
}
