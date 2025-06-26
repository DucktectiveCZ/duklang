use crate::parser::{BinOp, Expr, Parser};

fn extract_binary(expr: Expr) -> (BinOp, Box<Expr>, Box<Expr>) {
    if let Expr::Binary { op, left, right } = expr {
        (op, left, right)
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_add_mul_precedence() {
    let expr = Parser::new("1 + 2 * 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Add);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Mul);
}

#[test]
fn test_add_div_precedence() {
    let expr = Parser::new("1 + 2 / 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Add);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Div);
}

#[test]
fn test_add_mod_precedence() {
    let expr = Parser::new("1 + 2 % 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Add);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Mod);
}

#[test]
fn test_sub_mul_precedence() {
    let expr = Parser::new("1 - 2 * 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Sub);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Mul);
}

#[test]
fn test_sub_div_precedence() {
    let expr = Parser::new("1 - 2 / 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Sub);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Div);
}

#[test]
fn test_sub_mod_precedence() {
    let expr = Parser::new("1 - 2 % 3").parse_expr().unwrap();
    let (op, _, right) = extract_binary(expr);
    assert_eq!(op, BinOp::Sub);
    let (op_inner, _, _) = extract_binary(*right);
    assert_eq!(op_inner, BinOp::Mod);
}

#[test]
fn test_nested_precedence_all_ops() {
    let expr = Parser::new("1 + 2 * 3 - 4 / 5 % 6").parse_expr().unwrap();

    // Expect root is Subtraction
    let (op_root, left, right) = extract_binary(expr);
    assert_eq!(op_root, BinOp::Sub);

    // Left should be Add
    let (op_left, _, _) = extract_binary(*left);
    assert_eq!(op_left, BinOp::Add);

    // Right should be Mod
    let (op_right, _, _) = extract_binary(*right.clone());
    assert_eq!(op_right, BinOp::Mod);

    // And within Mod, left should be Div
    let (_, div_left, _) = extract_binary(*right);
    let (op_div, _, _) = extract_binary(*div_left);
    assert_eq!(op_div, BinOp::Div);
}
