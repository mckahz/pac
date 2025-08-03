use super::{expression::expression, *};
use crate::ast::source::{Constructor, Statement, TypeDef};

fn import_hierarchy(i: &str) -> Result<String> {
    alt((value_identifier, type_identifier))(i)
}

fn import_statement(i: &str) -> Result<Statement> {
    let (i, module) = delimited(keyword("import"), import_hierarchy, symbol(";"))(i)?;
    Ok((i, Statement::Import(Import { module })))
}

fn constructor(i: &str) -> Result<(String, Vec<Type>)> {
    let (i, name) = type_identifier(i)?;
    let (i, args) = many0(tipe::factor).parse(i)?;

    Ok((i, (name, args)))
}

fn internal_type_def(i: &str) -> Result<Statement> {
    let (i, name) = type_identifier(i)?;
    let (i, args) = many0(value_identifier).terminated(symbol("=")).parse(i)?;
    let (i, bar) = opt(symbol("|"))(i)?;
    let (i, constructors) = separated_list1(symbol("|"), constructor)
        .map(|constructors| {
            constructors
                .into_iter()
                .map(|(name, args)| Constructor { name, args })
                .collect()
        })
        .terminated(symbol(";"))
        .parse(i)?;

    Ok((
        i,
        Statement::LetType(name, TypeDef::Internal { args, constructors }),
    ))
}

fn external_type_def(i: &str) -> Result<Statement> {
    let (i, name) = type_identifier(i)?;
    let (i, _args) = many0(value_identifier).terminated(symbol("=")).parse(i)?;

    let (i, type_def) = expression::string_literal
        .preceded_by(keyword("extern"))
        .terminated(symbol(";"))
        .map(|s| TypeDef::External(s))
        .parse(i)?;

    Ok((i, Statement::LetType(name, type_def)))
}

fn let_type(i: &str) -> Result<Statement> {
    alt((internal_type_def, external_type_def))(i)
}

fn let_signature(i: &str) -> Result<Statement> {
    let (i, name) = value_identifier.terminated(symbol(":")).parse(i)?;
    let (i, tipe) = tipe::tipe.terminated(symbol(";")).parse(i)?;

    Ok((i, Statement::LetSignature(name, tipe)))
}

fn let_value(i: &str) -> Result<Statement> {
    let (i, name) = value_identifier(i)?;
    let (i, params) = many0(pattern::pattern).terminated(symbol("=")).parse(i)?;

    let (i, body) = expression.terminated(symbol(";")).parse(i)?;

    let rhs = if params.is_empty() {
        body
    } else {
        let mut params = params.into_iter().rev();
        let last_param = params.next().unwrap();
        params
            .into_iter()
            .fold(Expr::Lambda(last_param, Box::new(body)), |f, arg| {
                Expr::Lambda(arg, Box::new(f))
            })
    };

    Ok((i, Statement::LetValue(name, rhs)))
}

fn let_declaration(i: &str) -> Result<Statement> {
    preceded(
        keyword("let"),
        alt((
            let_value.context("let value"),
            let_signature.context("type signature"),
            let_type.context("type definition"),
        )),
    )
    .parse(i)
}

pub fn parse_statement(i: &str) -> Result<Statement> {
    alt((
        import_statement.context("import statement"),
        let_declaration,
    ))
    .parse(i)
}
