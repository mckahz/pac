mod expression;
mod pattern;
mod statement;
mod tipe;

use std::{collections::HashMap, fmt::write};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0, satisfy},
    combinator::{eof, fail, not, opt, success},
    error::ParseError,
    multi::{many0, many1, many_m_n, many_till, separated_list0, separated_list1},
    number,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use nom_supreme::{final_parser::final_parser, ParserExt};

use crate::{Expr, Import, Module, Project, Statement, Type};

pub type Result<'a, O> = IResult<&'a str, O, nom_supreme::error::ErrorTree<&'a str>>;

const KEYWORDS: [&str; 11] = [
    "if",
    "then",
    "else",
    "when",
    "is",
    "let",
    "module",
    "import",
    "crash",
    "dbg",
    "extern",
];

fn lexeme<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Result<O>
where
    F: Fn(&'a str) -> Result<O>,
{
    delimited(multispace0, inner, multispace0)
}

fn keyword<'a>(kw: &'static str) -> impl FnMut(&'a str) -> Result<&'a str> {
    lexeme(tag(kw))
}

fn symbol<'a>(s: &'a str) -> impl FnMut(&'a str) -> Result<&'a str> {
    lexeme(tag(s))
}

fn parens<'a, O>(p: impl FnMut(&'a str) -> Result<O>) -> impl FnMut(&'a str) -> Result<O> {
    delimited(symbol("("), p, symbol(")"))
}

fn value_identifier(i: &str) -> Result<String> {
    let chars = |i| {
        let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_lowercase()) || c == '_')(i)?;
        let (i, mut rest) = many0(satisfy(|c| {
            (c.is_alphabetic() && c.is_lowercase()) || c == '_' || c.is_numeric()
        }))(i)?;
        let (i, mut end) = many0(satisfy(|c| c == '?'))(i)?;
        let mut ident: Vec<char> = vec![first];
        ident.append(&mut rest);
        ident.append(&mut end);
        nom::IResult::Ok((
            i,
            ident
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""),
        ))
    };

    let (i, ident) = lexeme(chars)(i)?;
    if KEYWORDS.contains(&&*ident) {
        fail.context("non keyword identifier").parse(i)
    } else {
        success(ident).context("identifier").parse(i)
    }
}

fn type_identifier(i: &str) -> Result<String> {
    let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_uppercase()))(i)?;
    let (i, mut rest) = many0(satisfy(|c| c.is_alphabetic() || c.is_numeric()))(i)?;
    let (i, mut end) = many0(satisfy(|c| c == '?'))(i)?;
    let mut ident: Vec<char> = vec![first];
    ident.append(&mut rest);
    ident.append(&mut end);
    nom::IResult::Ok((
        i,
        ident
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(""),
    ))
}

pub fn file(i: &str) -> Result<Module> {
    let (i, module_name) = preceded(keyword("module"), alphanumeric1.context("module name"))(i)?;

    let (i, interface) = delimited(
        symbol("["),
        separated_list0(symbol(","), alt((type_identifier, value_identifier))),
        symbol("]"),
    )
    .terminated(symbol(";"))
    .parse(i)?;

    let (i, statements) = many_till(statement::parse_statement.context("statement"), eof)
        .map(|r| r.0)
        .parse(i)?;

    success(Module {
        name: module_name.to_owned(),
        interface,
        statements,
    })(i)
}
