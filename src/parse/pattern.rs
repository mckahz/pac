use crate::parse::ast::Pattern;

use super::*;


fn factor(i: &str) -> Result<Pattern> {
    alt((
        parens(pattern),
        symbol("_").map(|_| Pattern::Wildcard),
        move |i| {
            let (i, ident) = value_identifier(i)?;
            success(Pattern::Identifier(ident))(i)
        },
        symbol("[]").map(|_| Pattern::EmptyList),
    ))(i)
}

fn term(i: &str) -> Result<Pattern> {
    alt((
        parens(pattern),
        tuple((type_identifier, many0(factor)))
            .map(|(tag, args)| Pattern::Product(tag, args)),
        factor,
    ))(i)
}

pub fn pattern(i: &str) -> Result<Pattern> {
    let (i, terms) = many0(terminated(term, symbol("::")))(i)?;
    let (i, last) = term(i)?;
    if terms.is_empty() {
        return Ok((i, last));
    }
    let cons = terms
        .into_iter()
        .rev()
        .fold(last, |acc, t| Pattern::Cons(Box::new(t), Box::new(acc)));
    success(cons)(i)
}
