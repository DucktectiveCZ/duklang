use logos::Lexer;

use crate::tokenizer::Token;

#[derive(Debug)]
pub struct ObjectMetadata {
    pub name: String,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Class(ClassDecl),
    Fun(FunDecl),
    Let(LetDecl),
}

#[derive(Debug)]
pub struct ClassDecl {
    pub metadata: ObjectMetadata,
    pub visibility: Visibility,

    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct LetDecl {
    pub metadata: ObjectMetadata,
    pub type_name: String,
    pub visibility: Visibility,
}

#[derive(Debug)]
pub struct FunDecl {
    pub metadata: ObjectMetadata,
    pub visibility: Visibility,

    pub ret_type: String,
    pub args: Vec<ArgDecl>,
    pub code: Vec<Expr>,
}

#[derive(Debug)]
pub struct ArgDecl {
    pub metadata: ObjectMetadata,

    pub name: String,
    pub type_name: String,
}

#[derive(Debug)]
pub struct CallExpr {
    pub callee: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum OpExpr {
    Add { x: Box<Expr>, y: Box<Expr>, },
    Sub { x: Box<Expr>, y: Box<Expr>, },
    Mul { x: Box<Expr>, y: Box<Expr>, },
    Div { x: Box<Expr>, y: Box<Expr>, },
}

#[derive(Debug)]
pub enum LiteralExpr {
    Int(i32),
    Float(f32),
    Str(String),
}

#[derive(Debug)]
pub enum Expr {
    Call(CallExpr),
    Op(OpExpr),
    Literal(LiteralExpr),
}

#[derive(Debug)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub arm_then: Box<Expr>,
    pub arm_else: Box<Expr>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Expr,
}

pub fn parse(lexer: &Lexer<Token>) {
    ;
}
