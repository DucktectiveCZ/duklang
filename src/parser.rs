use logos::Lexer;

use crate::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Unknown")]
    Unknown,

    #[error("Expected {expected}, found {found}")]
    ExpectedDifferentToken { expected: Token, found: Token },

    #[error("Unknown")]
}

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
    Add { x: Box<Expr>, y: Box<Expr> },
    Sub { x: Box<Expr>, y: Box<Expr> },
    Mul { x: Box<Expr>, y: Box<Expr> },
    Div { x: Box<Expr>, y: Box<Expr> },
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

fn extract_ident(lexer: &mut Lexer<Token>) -> Result<String, String> {
    let tok = lexer.next();

    if !tok.is_some() {
        return Err("Expected attribute name, found EOF".to_string());
    }
    if !tok.unwrap().is_ok() {
       return Err("Expected attribute name, found Error".to_string());
    }

    if let Token::Ident(ident) = tok.unwrap().unwrap() {
        return ident;
    }
    
    Err(format!("Expected ident, found {:?}", tok.unwrap().unwrap()))
}

fn parse_fun(lexer: &mut Lexer<Token>) -> Result<FunDecl, String> {
    while let Some(token) = lexer.next() {
        let span = lexer.span();

        if let Err(_) = token {
            return Err(format!("Bad token at bytes {} - {}", span.start, span.end));
        }

        match token.unwrap() {
            Token::At => 
        }
    };
}

pub fn parse_module(lexer: &mut Lexer<Token>) -> Result<Module, String> {
    let mut new_module = Module { decls: vec![] };

    while let Some(token) = lexer.next() {
        if let Err(_) = token {
            let span = lexer.span();
            return Err(format!("Bad token at bytes {} - {}", span.start, span.end));
        }

        let new_node = match token.unwrap() {
            Token::Fun => {}
            _ => return Err("Token not implemented in this version".to_string()),
        };
        new_module.decls.push(new_node);
    }

    new_module.decls.shrink_to_fit();

    Ok(Module { decls: vec![] })
}
