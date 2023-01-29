use std::fs;
use std::env;
use logos::Logos;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
enum LexingError {
    InvalidInteger(String),
    #[default]
    NonAsciiCharacter,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("Overflow error!".to_owned()),
            _ => LexingError::InvalidInteger("Other error!".to_owned()),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos()]
enum Token {
    #[token("~")]
    Tilde,

    #[token("`")]
    Backtick,

    #[token("!")]
    Bang,

    #[token("@")]
    At,

    #[token("#")]
    Hash,

    #[token("$")]
    Dollar,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("&")]
    Amp,

    #[token("*")]
    Star,

    #[token("(")]
    LeftPar,

    #[token(")")]
    RightPar,

    #[token("-")]
    Minus,

    #[token("_", priority = 3)]
    Underscore,

    #[token("+")]
    Plus,

    #[token("=")]
    Eq,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("|")]
    Pipe,

    #[token("\\")]
    Backslash,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("\"")]
    Quote,

    #[token("'")]
    SingleQuote,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("?")]
    Query,

    #[token("/")]
    Slash,

    #[token(":?")]
    ColonQuery,

    #[token(":=")]
    ColonEq,

    #[token("==")]
    EqEq,

    #[token("=>")]
    EqGt,

    #[token("!=")]
    BangEq,

    #[token("<=")]
    LtEq,

    #[token("<-")]
    LtMinus,

    #[token("<.")]
    LtDot,

    #[token("<_")]
    LtUnderscore,

    #[token("</")]
    LtSlash,

    #[token("<\\")]
    LtBackslash,

    #[token(">=")]
    GtEq,

    #[token(">-")]
    GtMinus,

    #[token(">.")]
    GtDot,

    #[token(">_")]
    GtUnderscore,

    #[token(">/")]
    GtSlash,

    #[token(">\\")]
    GtBackslash,

    #[token("~=")]
    TildeEq,

    #[token("@=")]
    AtEq,

    #[token("#=")]
    HashEq,

    #[token("$=")]
    DollarEq,

    #[token("^=")]
    CaretEq,

    #[token("&=")]
    AmpEq,

    #[token("*=")]
    StarEq,

    #[token("-=")]
    MinusEq,

    #[token("->")]
    MinusGt,

    #[token("+=")]
    PlusEq,

    #[token("_=")]
    UnderscoreEq,

    #[token("_>")]
    UnderscoreGt,

    #[token("|=")]
    PipeEq,

    #[token("\\=")]
    BackslashEq,

    #[token("\\>")]
    BackslashGt,

    #[token("/=")]
    SlashEq,

    #[token("/>")]
    SlashGt,

    #[token(";=")]
    SemicolonEq,

    #[token(".=")]
    DotEq,

    #[token("..")]
    DotDot,

    #[token(".>")]
    DotGt,

    #[token(",=")]
    CommaEq,

    #[token("?=")]
    QueryEq,

    #[token("?:")]
    QueryColon,

    #[token("..=")]
    DotDotEq,

    #[token("...")]
    DotDotDot,

    #[token("i32")]
    I32,

    #[token("fn")]
    Fn,

    #[token("rt")]
    Rt,

    #[token("T", |_| true, priority = 3)]
    #[token("F", |_| false, priority = 3)]
    Bool(bool),

    #[regex(r"\n+", |lex| lex.slice().len())]
    NewLine(usize),

    #[regex(r"\t+", |lex| lex.slice().len())]
    Tab(usize),

    #[regex(r"[ ]+", |lex| lex.slice().len())]
    Space(usize),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Num(u64),

    // #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    // Num(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Name,

    #[regex("'[^']*'|\"[^\"]*\"")]
    String,
}

fn main() -> std::io::Result<()> {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument!")).expect("Failed to read file!");
    println!("Source:\n{}", src);

    let mut lex = Token::lexer(src.as_str());
    while let Some(token) = lex.next() {
        let mut value = lex.slice().to_string();
        let tok = token.unwrap();
        if tok != Token::String {
            value = value.replace(' ', "\\s").replace('\t', "\\t").replace('\n', "\\n");
        }
        println!("{}\n  {:?}", value, tok);
    }
    Ok(())
}
