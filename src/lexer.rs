use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, strum_macros::Display)]
pub enum Token {
    // Keywords
    #[token("fun")]
    Fun,
    #[token("class")]
    Class,
    #[token("let")]
    Let,
    #[token("pub")]
    Pub,
    #[token("priv")]
    Priv,
    #[token("import")]
    Import,
    #[token("group")]
    Group,
    #[token("ret")]
    Ret,
    #[token("new")]
    New,
    #[token("@")]
    At,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    // Literals
    #[regex(r"[0-9]([0-9_]*[0-9])?u")]
    UIntLiteral,
    #[regex(r"[0-9]([0-9_]*[0-9])?")]
    IntLiteral,
    #[regex(r"[0-9][_0-9]*\.[0-9][_0-9]*([eE][+-]?[0-9][_0-9]*)?", priority = 2)]
    FloatLiteral,
    #[regex(r#""([^"\\]|\\.)*""#)] // simple double quoted strings with escapes
    StrLiteral,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    #[token("=")]
    Equals,
    #[token("==")]
    EqualsEquals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEquals,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEquals,

    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // Punctuation
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    // Comments (skip)
    #[regex(r"//[^\n]*", logos::skip)]
    CommentLine,
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)]
    CommentBlock,

    // Whitespace (skip)
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}
