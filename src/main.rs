use anyhow::{anyhow, Context, Result};
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
enum Expr {
    // Self-Evaluations
    Number(i32),
    Nil,
    // Procedure
    Pair{car: Box<Expr>, cdr: Box<Expr>},
    OpAdd,
    OpMul,
    OpSub,
    OpDiv,
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;
        match self {
            Number(n) => write!(f, "{n}"),
            Nil => write!(f, "nil"),
            Pair{car, cdr} => write!(f, "({car} {cdr})"),
            OpAdd => write!(f, "+"),
            OpMul => write!(f, "*"),
            OpSub => write!(f, "-"),
            OpDiv => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    LParen,
    RParen,
    Num(i32),
    OpAdd,
    OpMul,
    OpSub,
    OpDiv,
    Unimplemented(String),
}

#[derive(Debug)]
enum ParseError {
    MissingLParen,
    MissingRParen,
    ExpressionFailed,
    ProcedureFailed,
    ListFailed,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            MissingLParen => write!(f, "MissingLeftParen"),
            MissingRParen => write!(f, "MissingRightParen"),
            ExpressionFailed => write!(f, "Failed parse expression"),
            ProcedureFailed => write!(f, "Failed parse procedure"),
            ListFailed => write!(f, "Failed parse list"),
        }
    }
}

impl Error for ParseError {}

fn str2token(code: &str) -> Token {
    match code {
        "(" => Token::LParen,
        ")" => Token::RParen,
        code if code.chars().all(char::is_numeric) => Token::Num(code.parse().unwrap()),
        "+" => Token::OpAdd,
        "-" => Token::OpSub,
        "*" => Token::OpMul,
        "/" => Token::OpDiv,
        _ => Token::Unimplemented(code.to_string()),
    }
}
fn tokenize(input: &str) -> Vec<Token> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(str2token)
        .collect()
}

fn check_paren_balance(tokens: &[Token]) -> Result<()> {
    let mut count = 0;
    for token in tokens {
        match token {
            Token::LParen => count += 1,
            Token::RParen => {
                count -= 1;
                if count < 0 {
                    return Err(ParseError::MissingLParen)?;
                }
            }
            _ => {}
        }
    }
    if count > 0 {
        Err(ParseError::MissingRParen)?
    } else {
        Ok(())
    }
}

fn parse_expr(tokens: &[Token]) -> Result<Expr> {
    let cur = &tokens[0];
    let rest = &tokens[1..];
    debug!("parse_expr {:?}", cur);
    match *cur {
        Token::LParen => parse_procedure(rest),
        Token::RParen => {
            Ok(Expr::Nil)
        }
        Token::Num(n) => Ok(Expr::Number(n)),
        _ => Err(ParseError::ExpressionFailed)?,
    }
}

fn parse_procedure(tokens: &[Token]) -> Result<Expr> {
    let cur = &tokens[0];
    let rest = &tokens[1..];
    debug!("parse_procedure {:?}", cur);
    match *cur {
        Token::OpAdd => Ok(Expr::Pair{
            car: Box::new(Expr::OpAdd),
            cdr: Box::new(parse_list(rest)?),
        }),
        Token::OpSub => Ok(Expr::Pair{
            car: Box::new(Expr::OpSub),
            cdr: Box::new(parse_list(rest)?),
        }),
        Token::Num(n) => Ok(Expr::Pair{
            car: Box::new(Expr::Number(n)),
            cdr: Box::new(parse_expr(rest)?),
        }),
        _ => Err(ParseError::ProcedureFailed)?,
    }
}

/// Parse list. '(' has already been read.
/// # Examples
/// ```
/// 1 2 3) // has already '(' been read.
/// >> Expr::Pair(1, Expr::Pair(2, Expr::Pair(3, Expr::Nil)))
///
/// (+ 1 2) 3)
/// >> Expr::Pair(
///     Expr("(+ 1 2)")
///     Expr("3")
/// )
/// >> Expr::Pair(
///         Expr::Pair(Expr::OpAdd, // (+ 1 2)
///                     Expr::Pair(1,
///                     Expr::Pair(2, Expr::Nil))),
///         Expr::Pair(3, Expr::Nil))
/// ```
fn parse_list(tokens: &[Token]) -> Result<Expr> {
    let list = Expr::Nil;
    for cur in tokens {
        debug!("parse_list {:?}", cur);
        let e = match *cur {
            Token::LParen => {
                Expr::Pair(
                    parse_procedure(rest)?,
                    parse_list(rest)?
                );
            },
            Token::RParen => {
                Ok(Expr::Nil),
            }
            Token::Num(n) => Ok(Expr::Pair(
                Box::new(Expr::Number(n)),
                Box::new(parse_list(rest)?),
            )),
            _ => Err(ParseError::ListFailed)?,
        }
    }
    Ok(list)
}

fn eval(expr: &Expr) -> Result<()> {
    unimplemented!()
}

use env_logger;
use log::{info, debug, error};
use std::env;
fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let code = "(+ (+ 2 3) (- 12 3))";
    let tokens = tokenize(code);
    info!("Code: {}", code);
    info!("Tokens: {:?}", tokens);
    match check_paren_balance(&tokens) {
        Ok(_) => {
            info!("Passed ParenCheck");
            let expr = parse_expr(&tokens);
            match expr {
                Ok(e) => println!("Expr: {e}"),
                Err(err) => println!("ExprError: {err}"),
            }
        }
        Err(err) => error!("{err}"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_success() {
        use Token::*;
        assert_eq!(tokenize("1"), vec![Num(1)]);
        assert_eq!(
            tokenize("(1 2 3)"),
            vec![LParen, Num(1), Num(2), Num(3), RParen]
        );
        assert_eq!(
            tokenize("(+ 2 3)"),
            vec![LParen, OpAdd, Num(2), Num(3), RParen]
        );
        assert_eq!(
            tokenize("(+ (+ 2 3) (- 12 3))"),
            vec![
                LParen,
                OpAdd,
                LParen,
                OpAdd,
                Num(2),
                Num(3),
                RParen,
                LParen,
                OpSub,
                Num(12),
                Num(3),
                RParen,
                RParen
            ]
        );
    }
    #[test]
    #[ignore = "Not yet implemented"]
    #[should_panic]
    fn tokenize_failed() {
        todo!("tokenize panic case")
    }
    #[test]
    #[ignore]
    fn parser_success() {
        let code = "(+ (+ 2 3) (- 12 3))";
        let mut tokens = tokenize(code);
        let expr = parse_expr(&tokens);
    }
}
