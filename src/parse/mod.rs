mod expression;
mod pattern;
mod statement;
mod tipe;

use crate::ast::{Located, Name, Position, Region, Span};
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0, satisfy},
    combinator::{eof, fail, not, opt, success},
    error::*,
    multi::{many0, many1, many_m_n, many_till, separated_list0, separated_list1},
    number,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    AsChar, Compare, IResult, Input, Parser,
};
use nom_locate::{position, LocatedSpan};

use crate::ast::source::{Expr, Import, Module, Statement, Type};

pub type Result<'a, O> = IResult<Span<'a>, O>;

const KEYWORDS: [&str; 11] = [
    "if", "then", "else", "when", "is", "let", "module", "import", "crash", "dbg", "extern",
];

pub fn located<'a, O, E, F>(mut parser: F) -> impl Parser<Span<'a>, Output = Located<O>, Error = E>
where
    F: Parser<Span<'a>, Output = O, Error = E>,
    E: ParseError<Span<'a>>,
{
    move |i: Span<'a>| {
        let (i, start) = position(i)?;
        let (i, inside) = parser.parse(i)?;
        let (i, end) = position(i)?;
        Ok((
            i,
            Located {
                region: Region {
                    start: Position {
                        line: start.location_line() as usize,
                        column: start.get_column(),
                    },
                    end: Position {
                        line: end.location_line() as usize,
                        column: end.get_column(),
                    },
                },
                inner: inside,
            },
        ))
    }
}

pub fn lexeme<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Output = O, Error = E>,
    <I as Input>::Item: AsChar,
    I: Input + Clone,
    E: ParseError<I>,
{
    delimited(multispace0, parser, multispace0)
}

fn keyword<'a, I, E>(kw: &'static str) -> impl Parser<I, Output = (), Error = E>
where
    <I as Input>::Item: AsChar,
    I: Input + Clone + Compare<&'static str>,
    E: ParseError<I>,
{
    lexeme(tag(kw)).map(|_| ())
}

fn symbol<'a, I, E>(s: &'static str) -> impl Parser<I, Output = (), Error = E>
where
    <I as Input>::Item: AsChar,
    I: Input + Clone + Compare<&'static str>,
    E: ParseError<I>,
{
    lexeme(tag(s)).map(|_| ())
}

fn parens<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Output = O, Error = E>,
    <I as Input>::Item: AsChar,
    I: Input + Clone + Compare<&'static str>,
    E: ParseError<I>,
{
    delimited(symbol("("), parser, symbol(")"))
}

fn value_identifier(i: Span) -> Result<Name> {
    lexeme(_value_identifier).parse(i)
}

fn _value_identifier(i: Span) -> Result<Name> {
    let chars = |i| {
        let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_lowercase()) || c == '_')(i)?;
        let (i, mut rest) = many0(satisfy(|c| {
            (c.is_alphabetic() && c.is_lowercase()) || c == '_' || c.is_numeric()
        }))
        .parse(i)?;
        let (i, mut end) = many0(satisfy(|c| c == '?')).parse(i)?;
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

    let (i, ident) = lexeme(chars).parse(i)?;
    if KEYWORDS.contains(&&*ident) {
        context("non keyword identifier", fail()).parse(i)
    } else {
        context("identifier", success(ident)).parse(i)
    }
}

fn type_identifier(i: Span) -> Result<Name> {
    lexeme(_type_identifier).parse(i)
}

fn _type_identifier(i: Span) -> Result<Name> {
    let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_uppercase()))(i)?;
    let (i, mut rest) = many0(satisfy(|c| c.is_alphabetic() || c.is_numeric())).parse(i)?;
    let (i, mut end) = many0(satisfy(|c| c == '?')).parse(i)?;
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

pub fn file(i: Span) -> Result<Module> {
    let (i, module_name) =
        preceded(keyword("module"), context("module name", type_identifier)).parse(i)?;

    let (i, interface) = terminated(
        delimited(
            symbol("["),
            separated_list0(symbol(","), alt((type_identifier, value_identifier))),
            symbol("]"),
        ),
        symbol(";"),
    )
    .parse(i)?;

    let (i, statements) = many_till(statement::parse_statement, eof)
        .map(|r| r.0)
        .parse(i)?;

    let mut imports = vec![];
    let mut type_defs = vec![];
    let mut signatures = vec![];
    let mut defs = vec![];

    for statement in statements {
        match statement {
            Statement::Import(import) => imports.push(import),
            Statement::LetSignature(binding, tipe) => signatures.push((binding, tipe)),
            Statement::LetValue(binding, expr) => defs.push((binding, expr)),
            Statement::LetType(binding, type_def) => type_defs.push((binding, type_def)),
        }
    }

    success(Module {
        name: module_name.to_owned(),
        imports,
        type_defs,
        signatures,
        interface,
        // TODO: properly parse the imports, then the definitions in a module
        defs,
    })
    .parse(i)
}
