use super::{expression::expression, *};
use crate::ast::source::{Constructor, Expr_, Statement, TypeDef};

fn import_hierarchy(i: Span) -> Result<String> {
    alt((value_identifier, type_identifier)).parse(i)
}

fn import_statement(i: Span) -> Result<Statement> {
    let (i, module) = delimited(keyword("import"), import_hierarchy, symbol(";")).parse(i)?;
    Ok((i, Statement::Import(Import { module })))
}

fn constructor(i: Span) -> Result<(String, Vec<Type>)> {
    let (i, name) = type_identifier(i)?;
    let (i, args) = many0(tipe::factor).parse(i)?;

    success((name, args)).parse(i)
}

fn internal_type_def(i: Span) -> Result<Statement> {
    let (i, name) = type_identifier(i)?;
    let (i, args) = terminated(many0(value_identifier), symbol("=")).parse(i)?;
    let (i, bar) = opt(symbol("|")).parse(i)?;
    let (i, constructors) = terminated(
        separated_list1(symbol("|"), constructor).map(|constructors| {
            constructors
                .into_iter()
                .map(|(name, args)| Constructor { name, args })
                .collect()
        }),
        symbol(";"),
    )
    .parse(i)?;

    Ok((
        i,
        Statement::LetType(name, TypeDef::Internal { args, constructors }),
    ))
}

fn external_type_def(i: Span) -> Result<Statement> {
    let (i, name) = type_identifier(i)?;
    let (i, _args) = terminated(many0(value_identifier), symbol("=")).parse(i)?;

    let (i, type_def) = delimited(keyword("extern"), expression::string_literal, symbol(";"))
        .map(|s| TypeDef::External(s))
        .parse(i)?;

    Ok((i, Statement::LetType(name, type_def)))
}

fn let_type(i: Span) -> Result<Statement> {
    alt((internal_type_def, external_type_def)).parse(i)
}

fn let_signature(i: Span) -> Result<Statement> {
    let (i, name) = terminated(value_identifier, symbol(":")).parse(i)?;
    let (i, tipe) = terminated(tipe::tipe, symbol(";")).parse(i)?;

    Ok((i, Statement::LetSignature(name, tipe)))
}

fn let_value(i: Span) -> Result<Statement> {
    let (i, name) = value_identifier(i)?;
    let (i, params) = terminated(many0(pattern::pattern), symbol("=")).parse(i)?;
    let (i, body) = terminated(expression, symbol(";")).parse(i)?;

    let rhs = if params.is_empty() {
        body
    } else {
        params.into_iter().rev().fold(body, |f, arg| {
            let region = arg.region.merge(&f.region);
            Located {
                inner: Expr_::Lambda(arg, Box::new(f)),
                region,
            }
        })
    };

    Ok((i, Statement::LetValue(name, rhs)))
}

fn let_declaration(i: Span) -> Result<Statement> {
    preceded(
        keyword("let"),
        alt((
            context("let value", let_value),
            context("type signature", let_signature),
            context("type definition", let_type),
        )),
    )
    .parse(i)
}

pub fn parse_statement(i: Span) -> Result<Statement> {
    alt((
        context("import statement", import_statement),
        let_declaration,
    ))
    .parse(i)
}
