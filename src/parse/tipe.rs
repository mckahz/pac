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

pub fn factor(i: &str) -> Result<Type> {
    alt((
        type_identifier.map(Type::Identifier),
        value_identifier.map(Type::Identifier),
        tuple_type,
        record_type,
        unit_type,
        parens(tipe),
    ))(i)
}

fn constructor(i: &str) -> Result<Type> {
    let (i, name) = alt((type_identifier, value_identifier))(i)?;
    let (i, factors) = many0(factor)(i)?;
    if factors.is_empty() {
        success(Type::Identifier(name))(i)
    } else {
        success(Type::Cons(name, factors))(i)
    }
}

pub fn term(i: &str) -> Result<Type> {
    alt((constructor, factor))(i)
}

fn function(i: &str) -> Result<Type> {
    let (i, terms) = separated_list1(symbol("->"), term).parse(i)?;
    let mut terms = terms.into_iter().rev();
    let last = terms.next().unwrap();
    success(terms.fold(last, |acc, a| Type::Fn(Box::new(a), Box::new(acc))))(i)
}

pub fn tipe(i: &str) -> Result<Type> {
    alt((function, term))(i)
}
