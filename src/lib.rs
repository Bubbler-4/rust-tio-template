use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SKCombinator {
    S,
    K,
    Ap(Vec<SKCombinator>),
}
pub use SKCombinator::*;

use nom::{
    IResult,
    character::complete::{char, multispace0},
    combinator::{map, value, eof},
    branch::alt,
    sequence::{delimited, preceded, terminated},
    multi::{many0, many1},
    error::{ParseError, ContextError, VerboseError, convert_error},
    Err,
};

fn wrap_or_strip(v: Vec<SKCombinator>) -> SKCombinator {
    match v.len() {
        0 => Ap(vec![S, K, K]),
        1 => v[0].clone(),
        _ => Ap(v)
    }
}

fn parse_item<'a, E: ParseError<&'a str> + ContextError<&'a str>>(s: &'a str) -> IResult<&'a str, SKCombinator, E> {
    let parse_s = value(S, char('S'));
    let parse_k = value(K, char('K'));
    let parse_paren = map(delimited(terminated(char('('), multispace0), many1(parse_item::<'a, E>), char(')')), wrap_or_strip);
    terminated(alt((parse_s, parse_k, parse_paren)), multispace0)(s)
}

pub fn parse_full<'a, E: ParseError<&'a str> + ContextError<&'a str>>(s: &'a str) -> IResult<&'a str, SKCombinator, E> {
    preceded(multispace0, terminated(map(many0(parse_item::<'a, E>), wrap_or_strip), eof))(s)
}

fn sk_tree_to_bcl(tree: &SKCombinator) -> String {
    match tree {
        S => "01".to_string(),
        K => "00".to_string(),
        Ap(v) => v.iter().fold("1".repeat(v.len()-1), |s, t| s + &sk_tree_to_bcl(t))
    }
}

/// Convert to Binary Combinatory Logic, in the form of string of zeroes and ones:
/// 2Ap X Y = 1XY; K = 00; S = 01
/// A rationale for partial byte being irrelevant is that the prefix notation terminates at
/// an unambiguous point, so the redundant bits can be ignored.
/// Returns Ok(bits) if input string is a valid SK expression, Err(msg) otherwise.

pub fn sk_to_bcl(s: &str) -> Result<String, String> {
    let parsed = parse_full::<VerboseError<&str>>(s);
    match parsed {
        Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(convert_error(s, e)),
        Ok((_, tree)) => Ok(sk_tree_to_bcl(&tree)),
        _ => Err("Unknown error".to_string())
    }
}

#[wasm_bindgen]
pub struct SKParseResult {
    _tree: Option<SKCombinator>,
    bcl: String,
    error_msg: String
}

#[wasm_bindgen]
impl SKParseResult {
    pub fn parse(s: &str) -> SKParseResult {
        let parsed = parse_full::<VerboseError<&str>>(s);
        let (tree, bcl, error_msg) = match parsed {
            Err(Err::Error(e)) | Err(Err::Failure(e)) =>
                (None, "".to_string(), convert_error(s, e)),
            Ok((_, tree)) =>
                { let bcl = sk_tree_to_bcl(&tree); (Some(tree), bcl, "".to_string()) },
            _ => (None, "".to_string(), "Unknown error".to_string())
        };
        SKParseResult { _tree: tree, bcl, error_msg }
    }

    pub fn bcl(&self) -> String {
        self.bcl.clone()
    }

    pub fn error_msg(&self) -> String {
        self.error_msg.clone()
    }
}
