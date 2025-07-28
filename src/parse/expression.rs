use std::collections::VecDeque;

use nom::{combinator::success, multi::separated_list0};
use nom_supreme::ParserExt;

use crate::parse::ast::{Assoc, Expr, Operator, Pattern};

use super::*;

fn identifier(i: &str) -> Result<Expr> {
    value_identifier
        .map(|s| Expr::Identifier(s.to_string()))
        .context("identifier")
        .parse(i)
}

fn constructor(i: &str) -> Result<Expr> {
    type_identifier
        .map(|s| Expr::Constructor(s.to_string()))
        .context("constructor")
        .parse(i)
}

fn nat(i: &str) -> Result<u32> {
    digit1
        .map(|digits: &str| digits.parse::<u32>().unwrap())
        .context("natural number")
        .parse(i)
}

fn int(i: &str) -> Result<i32> {
    terminated(
        tuple((
            opt(alt((tag("-"), tag("+")))).map(|sign| match sign {
                Some("-") => -1,
                Some("+") => 1,
                _ => 1,
            }),
            digit1.map(|string: &str| string.parse::<i32>().unwrap()),
        )),
        not(tag(".")),
    )
    .map(|(sign, num)| sign * num)
    .context("integer")
    .parse(i)
}

pub fn string_literal(i: &str) -> Result<String> {
    preceded(
        tag("\""),
        many_till(satisfy(|_| true), tag("\"")).map(|(chars, _)| {
            chars
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .to_owned()
        }),
    )
    .context("string literal")
    .parse(i)
}

fn record_literal(i: &str) -> Result<Expr> {
    delimited(
        symbol("{"),
        separated_list0(
            symbol(","),
            separated_pair(value_identifier, symbol(":"), expression),
        ),
        symbol("}"),
    )
    .map(|fields| Expr::Record(fields.into_iter().collect::<HashMap<String, Expr>>()))
    .context("record literal")
    .parse(i)
}

fn list(i: &str) -> Result<Expr> {
    delimited(
        symbol("["),
        separated_list0(symbol(","), expression),
        symbol("]"),
    )
    .map(Expr::List)
    .context("list")
    .parse(i)
}

fn factor(i: &str) -> Result<Expr> {
    alt((
        tuple((type_identifier.terminated(symbol(".")), value_identifier))
            .map(|(module, member)| Expr::Access(module, member)),
        parens(expression),
        record_literal,
        list,
        lexeme(string_literal).map(Expr::String),
        lexeme(nat).map(Expr::Nat),
        lexeme(int).map(Expr::Int),
        lexeme(number::complete::float).map(Expr::Float),
        lexeme(identifier),
        lexeme(constructor),
    ))
    .context("factor")
    .parse(i)
}

fn term(i: &str) -> Result<Expr> {
    let (i, func) = factor(i)?;

    let (i, args) = many0(factor)(i)?;

    if args.is_empty() {
        success(func)(i)
    } else {
        success(
            args.into_iter()
                .fold(func, |func, arg| Expr::Ap(Box::new(func), Box::new(arg))),
        )(i)
    }
}

fn operator(i: &str) -> Result<Operator> {
    use Operator::*;
    alt((
        symbol("<<").map(|_| Compose),
        symbol(">>").map(|_| ComposeRev),
        symbol("<|").map(|_| Pipe),
        symbol("|>").map(|_| PipeRev),
        symbol("||").map(|_| Or),
        symbol("&&").map(|_| And),
        symbol("==").map(|_| Eq),
        symbol("!=").map(|_| Neq),
        symbol("::").map(|_| Cons),
        symbol("<=").map(|_| LTE),
        symbol("<").map(|_| LT),
        symbol(">=").map(|_| GTE),
        symbol(">").map(|_| GT),
        symbol("++").map(|_| Concat),
        symbol("+").map(|_| Plus),
        symbol("-").map(|_| Minus),
        symbol("*").map(|_| Times),
        symbol("/").map(|_| Divide),
        symbol("%").map(|_| Mod),
        symbol("^").map(|_| Power),
    ))
    .context("operator")
    .parse(i)
}

fn operator_expression_help(
    ops: &mut VecDeque<Operator>,
    exprs: &mut VecDeque<Expr>,
    mut lhs: Expr,
    min_precedence: usize,
) -> Option<Expr> {
    let mut lookahead: Option<&Operator> = ops.front();

    while lookahead.is_some() && lookahead?.precedence() >= min_precedence {
        let op = ops.pop_front().unwrap();
        let mut rhs = exprs.pop_front().unwrap();
        lookahead = ops.front();

        while lookahead.is_some()
            && (lookahead?.precedence() > op.precedence()
                || (lookahead?.associativity() == Assoc::Right
                    && lookahead?.precedence() == op.precedence()))
        {
            rhs = operator_expression_help(
                ops,
                exprs,
                rhs,
                op.precedence()
                    + (if lookahead?.precedence() > op.precedence() {
                        1
                    } else {
                        0
                    }),
            )?;

            lookahead = ops.front();
        }

        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs.clone()),
        };
    }

    return Some(lhs);
}

fn operator_expression(i: &str) -> Result<Expr> {
    let (i, first) = term.parse(i)?;
    let (i, (mut ops, mut exprs)): (&str, (VecDeque<Operator>, VecDeque<Expr>)) =
        many1(tuple((operator, term)))
            .map(|pairs| pairs.into_iter().unzip())
            .parse(i)?;

    success(operator_expression_help(&mut ops, &mut exprs, first, 0).unwrap())(i)
}

fn let_from(i: &str) -> Result<Expr> {
    let (i, name) = value_identifier
        .preceded_by(keyword("let"))
        .terminated(symbol("<-"))
        .parse(i)?;
    let (i, value) = expression
        .terminated(symbol(";").context("semicolon"))
        .parse(i)?;
    let (i, body) = expression.parse(i)?;

    success(Expr::Bind(
        Pattern::Identifier(name),
        Box::new(value),
        Box::new(body),
    ))(i)
}

fn let_in(i: &str) -> Result<Expr> {
    let (i, name) = value_identifier
        .preceded_by(keyword("let"))
        .terminated(symbol("="))
        .parse(i)?;
    let (i, value) = expression
        .terminated(symbol(";").context("semicolon"))
        .parse(i)?;
    let (i, body) = expression.parse(i)?;

    success(Expr::Let(
        Pattern::Identifier(name),
        Box::new(value),
        Box::new(body),
    ))(i)
}

fn lambda(i: &str) -> Result<Expr> {
    let (i, args) = delimited(symbol("\\"), many1(pattern::pattern), symbol("->"))(i)?;
    let (i, body) = expression.parse(i)?;

    let mut args = args.into_iter().rev();
    let final_arg = args.next().unwrap();
    success(
        args.fold(Expr::Lambda(final_arg, Box::new(body)), |f, arg| {
            Expr::Lambda(arg, Box::new(f))
        }),
    )
    .context("lambda")
    .parse(i)
}

fn if_expr(i: &str) -> Result<Expr> {
    let (i, cond) = preceded(keyword("if"), expression)(i)?;
    let (i, then_branch) = preceded(keyword("then"), expression)(i)?;
    let (i, else_branch) = preceded(keyword("else"), expression)(i)?;
    success(Expr::If(
        Box::new(cond),
        Box::new(then_branch),
        Box::new(else_branch),
    ))(i)
}

fn alternative(i: &str) -> Result<(Pattern, Expr)> {
    use pattern::*;
    let (i, pat) = pattern.terminated(symbol("->")).parse(i)?;
    let (i, body) = expression(i)?;
    success((pat, body))(i)
}

fn when_expr(i: &str) -> Result<Expr> {
    let (i, val) = delimited(keyword("when"), expression, keyword("is"))(i)?;
    let (i, _) = opt(symbol("|"))(i)?;
    let (i, alternatives) = separated_list1(symbol("|"), alternative)
        .terminated(symbol(";"))
        .parse(i)?;
    success(Expr::When(Box::new(val), alternatives))(i)
}

fn crash_expr(i: &str) -> Result<Expr> {
    let (i, msg) = preceded(keyword("crash"), string_literal)(i)?;
    success(Expr::Ap(
        Box::new(Expr::External("crash".to_owned())),
        Box::new(Expr::String(msg)),
    ))(i)
}

fn extern_expr(i: &str) -> Result<Expr> {
    let (i, name) = preceded(keyword("extern"), string_literal)(i)?;
    success(Expr::External(name))(i)
}

fn tuple_expression(i: &str) -> Result<Expr> {
    let (i, first) = expression
        .preceded_by(symbol("("))
        .terminated(symbol(","))
        .parse(i)?;
    let (i, mut rest) = separated_list1(symbol(","), expression)
        .terminated(symbol(")"))
        .parse(i)?;
    let mut exprs = vec![first];
    exprs.append(&mut rest);
    Ok((i, Expr::Tuple(exprs)))
}

pub fn expression(i: &str) -> Result<Expr> {
    alt((
        if_expr,
        when_expr,
        lambda,
        crash_expr,
        extern_expr,
        operator_expression,
        let_from,
        let_in,
        term,
        tuple_expression,
        parens(expression),
    ))
    .context("expression")
    .parse(i)
}
