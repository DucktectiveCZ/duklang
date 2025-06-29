use std::num::{ParseFloatError, ParseIntError};

use logos::{Lexer, Logos};

use crate::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Unknown")]
    Unknown,

    #[error("Expected {expected}, found {found}")]
    ExpectedDifferentToken { expected: Token, found: Token },

    #[error("Expected {}, found {found}", expected.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "))]
    ExpectedDifferentTokens { expected: Vec<Token>, found: Token },

    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("Unknown token found")]
    ErrorToken,

    #[error("Expected token, found EOF")]
    ExpectedToken,

    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Missing terminating token: {0}")]
    MissingTerminatingToken(Token),

    #[error("A type annotation is missing")]
    MissingTypeAnnot,

    #[error("A code block is missing")]
    MissingCodeBlock,

    #[error("A assignment is missing")]
    MissingAssignment,
}

impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        Self::ErrorToken
    }
}

#[derive(Debug)]
pub enum VisibilityAnnot {
    Default, // Defaults to Private
    Private,
    Public,
}

type CodeBlock = Vec<RuntimeStatement>;

#[derive(Debug)]
pub struct Module {
    pub decls: Vec<GroupMemberStatement>,
}

#[derive(Debug)]
pub enum GroupMemberStatement {
    Class(ClassDeclStatement),
    Fun(FunDeclStatement),
    Let(ValDeclStatement),
}

#[derive(Debug)]
pub enum ClassMemberStatement {
    Fun(FunDeclStatement),
    Let(ValDeclStatement),
}

#[derive(Debug)]
pub enum RuntimeStatement {
    Let(ValDeclStatement),
    Discard(Expr),
    Return(Option<Expr>),
}

#[derive(Debug)]
pub struct ClassDeclStatement {
    pub attributes: Vec<String>,
    pub visibility: VisibilityAnnot,

    pub name: Option<String>,
    pub decls: Vec<GroupMemberStatement>,
}

#[derive(Debug)]
pub struct ValDeclStatement {
    pub attributes: Vec<String>,
    pub visibility: VisibilityAnnot,

    pub name: String,
    pub type_annot: Option<String>,
    pub initial_assignment: Option<Expr>,
}

#[derive(Debug)]
pub struct VarDeclStatement {
    pub attributes: Vec<AttributeAnnot>,
    pub visibility: VisibilityAnnot,

    pub name: String,
    pub type_annot: String,
    pub initial_assignment: Expr,
}

#[derive(Debug)]
pub struct FunDeclStatement {
    pub attributes: Vec<AttributeAnnot>,
    pub visibility: VisibilityAnnot,

    pub name: Option<String>,
    pub ret_type: Option<String>,
    pub args: Vec<ArgDecl>,
    pub code: CodeBlock,
}

#[derive(Debug)]
pub struct AttributeAnnot {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct ArgDecl {
    pub attributes: Vec<String>,

    pub name: String,
    pub type_name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinOp {
    Add, // x + y
    Sub, // x - y
    Mul, // x * y
    Div, // x / y
    Mod, // x % y

    Equals,       // x == y
    NotEquals,    // x != y
    Greater,      // x > y
    Lower,        // x < y
    GreaterEqual, // x >= y
    LowerEqual,   // x <= y

    Assign, // x = y
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,      // !x
    Positive, // +x
    Negative, // -x
    BitNot,   // ~x
}

#[derive(Debug, Clone)]
pub enum LiteralExpr {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Read(String),
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Unary {
        val: Box<Expr>,
        op: UnaryOp,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        op: BinOp,
    },
    Literal(LiteralExpr),
}

#[derive(Debug)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub arm_then: Box<Expr>,
    pub arm_else: Box<Expr>,
}

#[derive(Debug)]
pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    peeked: Option<Result<Token, ()>>,
    current_token: usize,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
            current_token: 0,
        }
    }

    fn next(&mut self) -> Option<Result<Token, ()>> {
        if let Some(tok) = self.peeked.take() {
            Some(tok)
        } else {
            self.current_token += 1;
            self.lexer.next()
        }
    }

    fn pop(&mut self) -> bool {
        self.next().is_some()
    }

    fn next_or_error(&mut self) -> Result<Token, ParseError> {
        Ok(self.next().ok_or(ParseError::ExpectedToken)??)
    }

    fn peek(&mut self) -> &Option<Result<Token, ()>> {
        if self.peeked.is_none() {
            self.peeked = self.next()
        }
        &self.peeked
    }

    fn peek_or_error(&mut self) -> Result<Token, ParseError> {
        Ok(self.peek().ok_or(ParseError::ExpectedToken)??)
    }

    fn slice(&mut self) -> &'source str {
        self.lexer.slice()
    }

    fn expect_next_token_to_be(&mut self, expected: Token) -> Result<(), ParseError> {
        let next_tok = self.peek_or_error()?;
        if next_tok == expected {
            Ok(())
        } else {
            Err(ParseError::ExpectedDifferentToken { expected, found: next_tok })
        }
    }

    pub fn parse_module(&mut self) -> Result<Module, ParseError> {
        let mut decls = Vec::new();
        while self.peek().is_some() {
            decls.push(self.parse_group_member_statement()?);
        }

        Ok(Module { decls })
    }

    pub fn parse_fun_decl(&mut self) -> Result<FunDeclStatement, ParseError> {
        let fun_tok = self.next().ok_or(ParseError::ExpectedToken)??;
        if fun_tok != Token::Fun {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::Fun,
                found: fun_tok,
            });
        }

        let name_tok = self.peek_or_error()?;
        let name = if name_tok == Token::Ident {
            let name_slice = self.slice();
            self.pop();
            Some(name_slice.to_string())
        } else {
            None
        };

        let args = self.parse_args_in_decl()?;

        let type_annot = self.parse_type_annot()?;

        let code = self.parse_code_block()?.ok_or(ParseError::MissingCodeBlock)?;

        Ok(FunDeclStatement {
            attributes: vec![],
            name,
            visibility: VisibilityAnnot::Default,
            ret_type: type_annot,
            args,
            code,
        })
    }

    // =<v>
    pub fn parse_assignment(&mut self) -> Result<Option<Expr>, ParseError> {
        if let Some(equals_tok_res) = self.peek() {
            let equals_tok = (*equals_tok_res)?;
            if equals_tok == Token::Equals {
                self.pop();
                let expr = self.parse_expr()?;
                return Ok(Some(expr));
            }
        }

        Ok(None)
    }

    // val<n>[:<T>][=<v>]
    pub fn parse_immutable_variable_decl(&mut self) -> Result<ValDeclStatement, ParseError> {
        let val_tok = self.next().ok_or(ParseError::ExpectedToken)??;
        if val_tok != Token::Val {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::Val,
                found: val_tok,
            });
        }

        let name_tok = self.next_or_error()?;
        if name_tok != Token::Ident {
            return Err(ParseError::ExpectedDifferentToken { expected: Token::Ident, found: name_tok }   )
        }
        let name_slice = self.slice();

        let type_annot = self.parse_type_annot()?;

        let initial_assignment = self.parse_assignment()?;

        Ok(ValDeclStatement{
            attributes: vec![], // TODO: Add attributes support
            visibility: VisibilityAnnot::Default, // TODO: Add visibility support
            name: name_slice.to_string(),
            type_annot,
            initial_assignment,
        })
    }

    // var<n>[:<T>][=<v>]
    pub fn parse_var_decl(&mut self) -> Result<VarDeclStatement, ParseError> {
        let attributes = self.parse_attribute_annots()?;

        let var_tok = self.next().ok_or(ParseError::ExpectedToken)??;
        if var_tok != Token::Var {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::Var,
                found: var_tok,
            });
        }

        let name_tok = self.next_or_error()?;
        if name_tok != Token::Ident {
            return Err(ParseError::ExpectedDifferentToken { expected: Token::Ident, found: name_tok })
        }
        let name_slice = self.slice();

        let type_annot = self.parse_type_annot()?.ok_or(ParseError::MissingTypeAnnot)?;

        let initial_assignment = self.parse_assignment()?.ok_or(ParseError::MissingAssignment)?;

        Ok(VarDeclStatement{
            attributes,
            visibility: VisibilityAnnot::Default, // TODO: Add visibility support
            name: name_slice.to_string(),
            type_annot,
            initial_assignment,
        })
    }

    pub fn parse_code_block(&mut self) -> Result<Option<Vec<RuntimeStatement>>, ParseError> {
        if *self.peek() != Some(Ok(Token::LeftBrace)) {
            return Ok(None);
        }
        self.pop();

        let mut statements = Vec::new();
        while self.peek_or_error()? != Token::RightBrace {
            statements.push(self.parse_runtime_statement()?);
            self.expect_next_token_to_be(Token::Semicolon)?;
            self.pop();
        }

        self.pop(); // Pop the terminating RightBrace

        Ok(Some(statements))
    }

    pub fn parse_attribute_annots(&mut self) -> Result<Vec<AttributeAnnot>, ParseError> {
        let mut annots = Vec::new();
        loop {
            let annot = self.parse_attribute_annot()?;
            if annot.is_none() {
                break;
            }
            annots.push(annot.unwrap());
        }
        Ok(annots)
    }

    pub fn parse_attribute_annot(&mut self) -> Result<Option<AttributeAnnot>, ParseError> {
        if *self.peek() != Some(Ok(Token::At)) {
            return Ok(None);
        }
        self.pop();

        let name_tok = self.next_or_error()?;
        if name_tok != Token::Ident {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::Ident,
                found: name_tok,
            });
        }
        let name_slice = self.slice();

        let left_paren_tok_opt = self.peek();
        let args = if left_paren_tok_opt.is_some() && left_paren_tok_opt.unwrap()? == Token::LeftParen {
            self.parse_args_in_call()?
        } else {
            vec![]
        };

        Ok(Some(AttributeAnnot { name: name_slice.to_string(), args}))
    }

    pub fn parse_visibility_annot(&mut self) -> Result<VisibilityAnnot, ParseError> {
        let visibility_tok = self.peek_or_error()?;
        let visibility = match visibility_tok {
            Token::Pub => VisibilityAnnot::Public,
            Token::Priv => VisibilityAnnot::Private,
            _ => VisibilityAnnot::Default,
        };
        self.next();

        Ok(visibility)
    }

    pub fn parse_type_annot(&mut self) -> Result<Option<String>, ParseError> {
        let colon_tok = self.peek_or_error()?;
        if colon_tok != Token::Colon {
            return Ok(None);
        }

        self.next();

        let type_tok = self.next().ok_or(ParseError::ExpectedToken)??;
        if type_tok != Token::Ident {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::Ident,
                found: type_tok,
            });
        }

        let type_slice = self.slice();

        Ok(Some(type_slice.to_string()))
    }

    pub fn parse_group_member_statement(&mut self) -> Result<GroupMemberStatement, ParseError> {
        let type_tok = self.peek_or_error()?;
        
        match type_tok {
            Token::Fun => Ok(GroupMemberStatement::Fun(self.parse_fun_decl()?)),
            Token::Val => todo!(),
            Token::Class => todo!(),
            t => Err(ParseError::UnexpectedToken(t)),
        }
    }

    pub fn parse_runtime_statement(&mut self) -> Result<RuntimeStatement, ParseError> {
        let type_tok = self.peek_or_error()?;
        
        match type_tok {
            Token::Val => todo!(),
            t => Err(ParseError::UnexpectedToken(t)),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_expr(0)
    }

    fn parse_int_literal(&self, slice: &str) -> Result<Expr, ParseError> {
        let num_str = slice.replace("_", "");
        let num: i64 = num_str.parse()?;

        Ok(Expr::Literal(LiteralExpr::Int(num)))
    }

    fn parse_uint_literal(&self, slice: &str) -> Result<Expr, ParseError> {
        let mut num_str = slice.to_string();
        num_str.pop();
        num_str = num_str.replace("_", "");
        let num: u64 = num_str.parse()?;

        Ok(Expr::Literal(LiteralExpr::UInt(num)))
    }

    fn parse_float_literal(&self, slice: &str) -> Result<Expr, ParseError> {
        let num_str = slice.replace("_", "");

        let value: f64 = num_str.parse()?;

        Ok(Expr::Literal(LiteralExpr::Float(value)))
    }

    fn parse_str_literal(&self, slice: &str) -> Result<Expr, ParseError> {
        let value = &slice[1..slice.len() - 1];

        Ok(Expr::Literal(LiteralExpr::Str(value.to_string())))
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary_expr()?;

        while let Some(Ok(op_token)) = self.peek() {
            let (op, prec) = match op_token {
                Token::Plus => (BinOp::Add, 4),
                Token::Minus => (BinOp::Sub, 4),
                Token::Star => (BinOp::Mul, 5),
                Token::Slash => (BinOp::Div, 5),
                Token::Percent => (BinOp::Mod, 5),

                Token::EqualsEquals => (BinOp::Equals, 2),
                Token::NotEquals => (BinOp::NotEquals, 2),
                Token::GreaterThanEquals => (BinOp::GreaterEqual, 2),
                Token::LessThanEquals => (BinOp::LowerEqual, 2),
                Token::GreaterThan => (BinOp::Greater, 3),
                Token::LessThan => (BinOp::Lower, 3),
                Token::Equals => (BinOp::Assign, 1),
                _ => break,
            };

            if prec < min_prec {
                break;
            }

            self.next();

            let right = self.parse_binary_expr(prec + 1)?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_args_in_decl(&mut self) -> Result<Vec<ArgDecl>, ParseError> {
        let left_paren_tok = self.next_or_error()?;
        if left_paren_tok != Token::LeftParen {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::LeftParen,
                found: left_paren_tok,
            });
        }

        let mut args = Vec::new();
        loop {
            let token = self.peek_or_error()?;
            match token {
                Token::RightParen => {
                    self.next();
                    return Ok(args);
                }
                Token::Ident => {
                    let name_slice = self.slice();
                    self.pop(); // pop the name token

                    let type_annot = self.parse_type_annot()?.ok_or(ParseError::MissingTypeAnnot)?;

                    let comma_tok = self.peek_or_error()?;
                    if comma_tok == Token::Comma {
                        self.pop();
                    } else if comma_tok != Token::RightParen {
                        return Err(ParseError::ExpectedDifferentTokens {
                            expected: vec![Token::Comma, Token::RightParen],
                            found: comma_tok,
                        });
                    }

                    args.push(ArgDecl { attributes: vec![], name: name_slice.to_string(), type_name: type_annot });
                }
                t => return Err(ParseError::UnexpectedToken(t))
            };
        }
    }

    fn parse_args_in_call(&mut self) -> Result<Vec<Expr>, ParseError> {
        let left_paren_tok = self.next_or_error()?;
        if left_paren_tok != Token::LeftParen {
            return Err(ParseError::ExpectedDifferentToken {
                expected: Token::LeftParen,
                found: left_paren_tok,
            });
        }

        let mut args = Vec::new();
        loop {
            let token = self.peek_or_error()?;
            match token {
                Token::RightParen => {
                    self.next();
                    return Ok(args);
                }
                _ => {
                    args.push(self.parse_expr()?);

                    let comma_tok = self.peek_or_error()?;
                    if comma_tok == Token::Comma {
                        self.pop();
                    } else if comma_tok != Token::RightParen {
                        return Err(ParseError::ExpectedDifferentTokens {
                            expected: vec![Token::Comma, Token::RightParen],
                            found: comma_tok,
                        });
                    }
                }
            };
        }
    }

    fn parse_ident(&mut self, slice: &str) -> Result<Expr, ParseError> {
        if let Some(next_tok_res) = self.peek() {
            let next_tok = (*next_tok_res)?;
            if next_tok == Token::LeftParen {
                let args = self.parse_args_in_call()?;
                return Ok(Expr::Call {
                    callee: slice.to_string(),
                    args,
                });
            }
        }

        Ok(Expr::Read(slice.to_string()))
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.next().ok_or(ParseError::ExpectedToken)??;
        let slice = self.slice();

        match token {
            Token::IntLiteral => self.parse_int_literal(slice),
            Token::UIntLiteral => self.parse_uint_literal(slice),
            Token::FloatLiteral => self.parse_float_literal(slice),
            Token::StrLiteral => self.parse_str_literal(slice),

            Token::Ident => self.parse_ident(slice),
                                                                
            t => Err(ParseError::UnexpectedToken(t)),
        }
    }
}
