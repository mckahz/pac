use crate::ast::source::{Type, Type_};

use super::*;

fn unit_type(i: Span) -> Result<Type> {
    located(symbol("()").map(|_| Type_::Unit)).parse(i)
}

fn record_type(i: Span) -> Result<Type> {
    located(delimited(
        symbol("{"),
        separated_list1(
            symbol(","),
            tuple((value_identifier, symbol(":"), tipe)).map(|(i, _, t)| (i, t)),
        )
        .map(|fields| Type_::Record(fields.into_iter().collect::<HashMap<String, Type>>())),
        symbol("}"),
    ))
    .parse(i)
}

fn tuple_type(i: Span) -> Result<Type> {
    located(parens(separated_list1(symbol(","), tipe)).map(|types| Type_::Tuple(types))).parse(i)
}

pub fn factor(i: Span) -> Result<Type> {
    alt((
        located(type_identifier.map(Type_::Identifier)),
        located(value_identifier.map(Type_::Identifier)),
        tuple_type,
        record_type,
        unit_type,
        parens(tipe),
    ))
    .parse(i)
}

fn constructor(i: Span) -> Result<Type> {
    located(constructor_help).parse(i)
}

fn constructor_help(i: Span) -> Result<Type_> {
    let (i, name) = alt((type_identifier, value_identifier)).parse(i)?;
    let (i, factors) = many0(factor).parse(i)?;
    if factors.is_empty() {
        success(Type_::Identifier(name)).parse(i)
    } else {
        success(Type_::Cons(name, factors)).parse(i)
    }
}

pub fn term(i: Span) -> Result<Type> {
    alt((constructor, factor)).parse(i)
}

fn function(i: Span) -> Result<Type> {
    separated_list1(symbol("->"), term)
        .map(|terms| {
            let mut terms = terms.into_iter().rev();
            let last = terms.next().unwrap();
            terms.fold(last, |acc, a| Located {
                region: a.region.merge(&acc.region),
                inner: Type_::Fn(Box::new(a), Box::new(acc)),
            })
        })
        .parse(i)
}

pub fn tipe(i: Span) -> Result<Type> {
    alt((function, term)).parse(i)
}
