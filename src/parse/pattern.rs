use crate::ast::source::{Pattern, Pattern_};

use super::*;

fn tuple_pattern(i: Span) -> Result<Pattern> {
    located(tuple_pattern_help).parse(i)
}

fn tuple_pattern_help(i: Span) -> Result<Pattern_> {
    let (i, first) = delimited(symbol("("), pattern, symbol(",")).parse(i)?;
    let (i, mut rest) = terminated(separated_list1(symbol(","), pattern), symbol(")")).parse(i)?;
    let mut elements = vec![];
    elements.push(first);
    elements.append(&mut rest);
    Ok((i, Pattern_::Tuple(elements)))
}

fn term(i: Span) -> Result<Pattern> {
    alt((
        located(symbol("_").map(|_| Pattern_::Wildcard)),
        located(symbol("[]").map(|_| Pattern_::Constructor("Empty".to_owned(), vec![]))),
        tuple_pattern,
        parens(pattern),
        located(value_identifier.map(|ident| Pattern_::Identifier(ident))),
        located(
            tuple((type_identifier, many0(term)))
                .map(|(tag, args)| Pattern_::Constructor(tag, args)),
        ),
    ))
    .parse(i)
}

pub fn pattern(i: Span) -> Result<Pattern> {
    let (i, mut terms) = separated_list1(symbol("::"), term).parse(i)?;
    let last = terms.pop().unwrap();
    let cons = terms.into_iter().fold(last, |acc, t| Located {
        region: acc.region.merge(&t.region),
        inner: Pattern_::Cons(Box::new(t), Box::new(acc)),
    });
    success(cons).parse(i)
}
