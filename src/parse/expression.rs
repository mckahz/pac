use std::collections::VecDeque;

use nom::{combinator::success, multi::separated_list0};

use crate::ast::source::{Assoc, Expr, Expr_, Operator, Pattern, Pattern_};

use super::*;

fn identifier(i: Span) -> Result<Expr> {
    located(value_identifier.map(|s| Expr_::Identifier(s))).parse(i)
}

fn constructor(i: Span) -> Result<Expr> {
    located(type_identifier.map(|s| Expr_::Constructor(s))).parse(i)
}

fn nat(i: Span) -> Result<u32> {
    lexeme(digit1)
        .map(|digits: Span| digits.parse::<u32>().unwrap())
        .parse(i)
}

fn int(i: Span) -> Result<i32> {
    lexeme(terminated(
        tuple((
            alt((tag("-").map(|_| -1), tag("+").map(|_| 1), success(1))),
            digit1.map(|string: Span| string.fragment().parse::<i32>().unwrap()),
        )),
        not(tag(".")),
    ))
    .map(|(sign, num)| sign * num)
    .parse(i)
}

pub fn string_literal(i: Span) -> Result<String> {
    lexeme(preceded(
        tag("\""),
        many_till(satisfy(|_| true), tag("\"")).map(|(chars, _)| {
            chars
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .to_owned()
        }),
    ))
    .parse(i)
}

fn record_literal(i: Span) -> Result<Expr> {
    located(
        delimited(
            symbol("{"),
            separated_list0(
                symbol(","),
                separated_pair(value_identifier, symbol(":"), expression),
            ),
            symbol("}"),
        )
        .map(|fields| Expr_::Record(fields.into_iter().collect::<HashMap<String, Expr>>())),
    )
    .parse(i)
}

fn list(i: Span) -> Result<Expr> {
    located(
        delimited(
            symbol("["),
            separated_list0(symbol(","), expression),
            symbol("]"),
        )
        .map(Expr_::List),
    )
    .parse(i)
}

fn factor(i: Span) -> Result<Expr> {
    alt((
        located(
            tuple((terminated(type_identifier, symbol(".")), value_identifier))
                .map(|(module, member)| Expr_::Access(module, member)),
        ),
        parens(expression),
        record_literal,
        list,
        identifier,
        constructor,
        located((string_literal).map(Expr_::String)),
        located((nat).map(Expr_::Nat)),
        located((int).map(Expr_::Int)),
        located(lexeme(number::complete::float).map(Expr_::Float)),
    ))
    .parse(i)
}

fn term(i: Span) -> Result<Expr> {
    let (i, func) = factor(i)?;
    let (i, args) = many0(factor).parse(i)?;

    if args.is_empty() {
        success(func).parse(i)
    } else {
        let ap = args.into_iter().fold(func, |func, arg| {
            let region = func.region.merge(&arg.region);
            Located {
                inner: Expr_::Ap(Box::new(func), Box::new(arg)),
                region,
            }
        });
        success(ap).parse(i)
    }
}

fn operator(i: Span) -> Result<Operator> {
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

        let region = lhs.region.merge(&rhs.region);
        lhs = Located {
            inner: Expr_::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs.clone()),
            },
            region,
        };
    }

    return Some(lhs);
}

fn operator_expression(i: Span) -> Result<Expr> {
    let (i, first) = term.parse(i)?;
    let (i, (mut ops, mut exprs)): (Span, (VecDeque<Operator>, VecDeque<Expr>)) =
        many1(tuple((operator, term)))
            .map(|pairs| pairs.into_iter().unzip())
            .parse(i)?;

    success(operator_expression_help(&mut ops, &mut exprs, first, 0).unwrap()).parse(i)
}

fn let_from(i: Span) -> Result<Expr> {
    located(let_from_help).parse(i)
}

fn let_from_help(i: Span) -> Result<Expr_> {
    let (i, name) = located(delimited(keyword("let"), value_identifier, symbol("<-"))).parse(i)?;
    let (i, value) = terminated(expression, symbol(";")).parse(i)?;
    let (i, body) = expression.parse(i)?;

    success(Expr_::Bind(
        Located {
            inner: Pattern_::Identifier(name.inner),
            region: name.region,
        },
        Box::new(value),
        Box::new(body),
    ))
    .parse(i)
}

fn let_in(i: Span) -> Result<Expr> {
    located(let_in_help).parse(i)
}

fn let_in_help(i: Span) -> Result<Expr_> {
    let (i, name) = located(delimited(keyword("let"), value_identifier, symbol("="))).parse(i)?;
    let (i, value) = terminated(expression, symbol(";")).parse(i)?;
    let (i, body) = expression.parse(i)?;

    success(Expr_::Let(
        name.map(|n| Pattern_::Identifier(n)),
        Box::new(value),
        Box::new(body),
    ))
    .parse(i)
}

fn lambda(i: Span) -> Result<Expr> {
    let (i, args) = delimited(symbol("\\"), many1(pattern::pattern), symbol("->")).parse(i)?;
    let (i, body) = expression.parse(i)?;

    let mut args = args.into_iter().rev();
    let final_arg = args.next().unwrap();
    success(args.fold(
        Located {
            region: final_arg.region.merge(&body.region),
            inner: Expr_::Lambda(final_arg, Box::new(body)),
        },
        |f, arg| Located {
            region: arg.region.merge(&f.region),
            inner: Expr_::Lambda(arg, Box::new(f)),
        },
    ))
    .parse(i)
}

fn if_expr(i: Span) -> Result<Expr> {
    located(if_expr_help).parse(i)
}

fn if_expr_help(i: Span) -> Result<Expr_> {
    let (i, cond) = preceded(keyword("if"), expression).parse(i)?;
    let (i, then_branch) = preceded(keyword("then"), expression).parse(i)?;
    let (i, else_branch) = preceded(keyword("else"), expression).parse(i)?;
    success(Expr_::If(
        Box::new(cond),
        Box::new(then_branch),
        Box::new(else_branch),
    ))
    .parse(i)
}

fn alternative(i: Span) -> Result<(Pattern, Expr)> {
    use pattern::*;
    let (i, pat) = terminated(pattern, symbol("->")).parse(i)?;
    let (i, body) = expression.parse(i)?;
    success((pat, body)).parse(i)
}

fn when_expr(i: Span) -> Result<Expr> {
    located(when_expr_help).parse(i)
}

fn when_expr_help(i: Span) -> Result<Expr_> {
    let (i, val) = delimited(keyword("when"), expression, keyword("is")).parse(i)?;
    let (i, _) = opt(symbol("|")).parse(i)?;
    let (i, alternatives) =
        terminated(separated_list1(symbol("|"), alternative), symbol(";")).parse(i)?;
    success(Expr_::When(Box::new(val), alternatives)).parse(i)
}

fn crash_expr(i: Span) -> Result<Expr> {
    let (i, kw) =
        located(keyword("crash").map(|_| Expr_::External("crash".to_owned()))).parse(i)?;
    let (i, msg) = located(string_literal.map(|s| Expr_::String(s))).parse(i)?;
    success(Located {
        region: kw.region.merge(&msg.region),
        inner: Expr_::Ap(Box::new(kw), Box::new(msg)),
    })
    .parse(i)
}

fn extern_expr(i: Span) -> Result<Expr> {
    located(preceded(keyword("extern"), string_literal).map(|name| Expr_::External(name))).parse(i)
}

fn tuple_expression(i: Span) -> Result<Expr> {
    located(tuple_expression_help).parse(i)
}

fn tuple_expression_help(i: Span) -> Result<Expr_> {
    let (i, first) = delimited(symbol("("), expression, symbol(",")).parse(i)?;
    let (i, mut rest) =
        terminated(separated_list1(symbol(","), expression), symbol(")")).parse(i)?;
    let mut exprs = vec![first];
    exprs.append(&mut rest);
    Ok((i, Expr_::Tuple(exprs)))
}

pub fn expression(i: Span) -> Result<Expr> {
    context(
        "expression",
        alt((
            // Keyword Expressions
            if_expr,
            when_expr,
            lambda,
            crash_expr,
            extern_expr,
            let_from,
            let_in,
            // Composite Expressions
            operator_expression,
            term,
            tuple_expression,
            parens(expression),
        )),
    )
    .parse(i)
}
