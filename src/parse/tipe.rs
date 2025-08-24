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
    located(
        parens(tuple((
            terminated(tipe, symbol(",")),
            tipe,
            preceded(symbol(","), separated_list0(symbol(","), tipe)),
        )))
        .map(|(first, second, rest)| Type_::Tuple(Box::new(first), Box::new(second), rest)),
    )
    .parse(i)
}

fn qualified(i: Span) -> Result<Type> {
    located(qualified_help).parse(i)
}

fn qualified_help(i: Span) -> Result<Type_> {
    let (i, mut module) = module_name.parse(i)?;
    match module.0.pop() {
        Some(constructor) if module.0.is_empty() => {
            success(Type_::Identifier(constructor)).parse(i)
        }
        Some(constructor) => success(Type_::QualifiedIdentifier(module, constructor)).parse(i),
        _ => fail().parse(i),
    }
}

pub fn factor(i: Span) -> Result<Type> {
    alt((
        qualified,
        located(value_identifier.map(Type_::Variable)),
        tuple_type,
        record_type,
        unit_type,
        parens(tipe),
    ))
    .parse(i)
}

pub fn term(i: Span) -> Result<Type> {
    located(term_help).parse(i)
}

fn term_help(i: Span) -> Result<Type_> {
    let (i, cons) = factor.parse(i)?;
    let (i, mut factors) = many0(factor).parse(i)?;
    factors.reverse();
    match factors.pop() {
        None => success(cons.inner).parse(i),
        Some(first_arg) => {
            factors.reverse();
            success(Type_::Constructor(
                Box::new(cons),
                Box::new(first_arg),
                factors,
            ))
            .parse(i)
        }
    }
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
