use crate::ast::source::Pattern;

use super::*;

fn tuple_pattern(i: &str) -> Result<Pattern> {
    let (i, first) = pattern
        .preceded_by(symbol("("))
        .terminated(symbol(","))
        .parse(i)?;
    let (i, mut rest) = separated_list1(symbol(","), pattern)
        .terminated(symbol(")"))
        .parse(i)?;
    let mut elements = vec![];
    elements.push(first);
    elements.append(&mut rest);
    Ok((i, Pattern::Tuple(elements)))
}

fn term(i: &str) -> Result<Pattern> {
    alt((
        symbol("_").map(|_| Pattern::Wildcard),
        symbol("[]").map(|_| Pattern::EmptyList),
        tuple_pattern,
        parens(pattern),
        value_identifier.map(|ident| Pattern::Identifier(ident)),
        tuple((type_identifier, many0(term))).map(|(tag, args)| Pattern::Constructor(tag, args)),
    ))(i)
}

pub fn pattern(i: &str) -> Result<Pattern> {
    let (i, mut terms) = separated_list1(symbol("::"), term).parse(i)?;
    let last = terms.pop().unwrap();
    let cons = terms.into_iter().fold(last, |acc, t| {
        Pattern::Constructor("Cons".to_owned(), vec![t, acc])
    });
    success(cons)(i)
}
