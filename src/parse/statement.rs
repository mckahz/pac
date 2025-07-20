use nom_supreme::ParserExt;

use self::expression::expression;

use super::*;

fn import_hierarchy(i: &str) -> Result<String> {
    alt((value_identifier, type_identifier))(i)
}

fn import_statement(i: &str) -> Result<Statement> {
    let (i, module) = delimited(keyword("import"), import_hierarchy, symbol(";"))(i)?;
    Ok((i, Statement::Import(Import { module, alias: None, children: vec![] })))
}

fn let_type(i: &str) -> Result<Statement> {
    let (i, name) = type_identifier(i)?;
    let (i, params) = many0(value_identifier).terminated(symbol("=")).parse(i)?;
    let (i, body) = tipe::tipe
        .terminated(symbol(";").context("semicolon"))
        .parse(i)?;

    let tipe = if params.is_empty() {
        body
    } else {
        let mut params = params.into_iter();
        let last = params.next().unwrap();
        params.fold(
            Type::Cons(Box::new(Type::Identifier(last)), Box::new(body)),
            |f, param| Type::Cons(Box::new(Type::Identifier(param)), Box::new(f)),
        )
    };

    success(Statement::Type(name, tipe))(i)
}

fn let_signature(i: &str) -> Result<Statement> {
    let (i, name) = value_identifier.terminated(symbol(":")).parse(i)?;
    let (i, tipe) = tipe::tipe
        .terminated(symbol(";").context("semicolon"))
        .parse(i)?;

    success(Statement::Signature(name, tipe))(i)
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

    success(Statement::Let(name, rhs))(i)
}

fn let_declaration(i: &str) -> Result<Statement> {
    preceded(
        keyword("let"),
        alt((
            let_value.context("let value"),
            let_signature.context("let signature"),
            let_type.context("let type"),
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
