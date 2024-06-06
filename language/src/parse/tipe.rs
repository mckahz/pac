use crate::PrettyPrint;

use super::*;

fn unit_type(i: &str) -> Result<Type> {
    symbol("()").map(|_| Type::Unit).parse(i)
}

fn record_type(i: &str) -> Result<Type> {
    delimited(
        symbol("{"),
        separated_list1(
            symbol(","),
            tuple((value_identifier, symbol(":"), tipe)).map(|(i, _, t)| (i, t)),
        )
        .map(|fields| Type::Record(fields.into_iter().collect::<HashMap<String, Type>>())),
        symbol("}"),
    )
    .parse(i)
}

fn tuple_type(i: &str) -> Result<Type> {
    let (i, types) = parens(separated_list1(symbol(","), tipe))(i)?;
    success(Type::Tuple(types))(i)
}

fn factor(i: &str) -> Result<Type> {
    alt((
        parens(tipe),
        tuple_type,
        record_type,
        unit_type,
        keyword("Nat").map(|_| Type::Nat),
        keyword("Int").map(|_| Type::Int),
        keyword("Float").map(|_| Type::Float),
        type_identifier.map(Type::Identifier),
        value_identifier.map(Type::Identifier),
    ))(i)
}

fn term(i: &str) -> Result<Type> {
    let (i, factors) = many1(factor)(i)?;
    success(Type::Product(factors))(i)
}

fn external(i: &str) -> Result<Type> {
    preceded(keyword("extern"), expression::string_literal).map(Type::External).parse(i)
}

fn function(i: &str) -> Result<Type> {
    let (i, terms) = separated_list1(symbol("->"), term).parse(i)?;
    let mut terms = terms.into_iter().rev();
    let last = terms.next().unwrap();
    success(
        terms.fold(last, |acc, a| Type::Fn(Box::new(a), Box::new(acc))),
    )(i)
}

fn sum(i: &str) -> Result<Type> {
    let (i, _) = symbol("|")(i)?;
    let (i, variants) = separated_list0(symbol("|"), term)(i)?;
    success(Type::Sum(variants))(i)
}

pub fn internal(i: &str) -> Result<Type> {
    alt((sum, function))(i)
}

pub fn tipe(i: &str) -> Result<Type> {
    alt((external, internal))(i)
}
