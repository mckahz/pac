use super::ast;
use super::*;
use crate::parse;

fn canonicalize_expression(
    parse_expr: parse::ast::Expr,
    type_defs: &[(String, parse::ast::TypeDef)],
) -> ast::Expr {
    match parse_expr {
        parse::ast::Expr::External(name) => ast::Expr::Extern(name),
        parse::ast::Expr::Let(pattern, expr, body) => match pattern {
            parse::ast::Pattern::Identifier(ident) => ast::Expr::Let {
                defs: vec![(ident, canonicalize_expression(*expr, type_defs))],
                body: Box::new(canonicalize_expression(*body, type_defs)),
            },
            _ => todo!(),
        },
        parse::ast::Expr::Bind(pattern, expr, expr1) => todo!(),
        parse::ast::Expr::If(cond, t, f) => ast::Expr::If {
            cond: Box::new(canonicalize_expression(*cond, type_defs)),
            true_branch: Box::new(canonicalize_expression(*t, type_defs)),
            false_branch: Box::new(canonicalize_expression(*f, type_defs)),
        },
        parse::ast::Expr::Ap(expr, expr1) => ast::Expr::Ap {
            function: Box::new(canonicalize_expression(*expr, type_defs)),
            arg: Box::new(canonicalize_expression(*expr1, type_defs)),
        },
        parse::ast::Expr::Identifier(ident) => ast::Expr::Binding(to_camel_case(&ident)),
        parse::ast::Expr::Lambda(pattern, expr) => match pattern {
            parse::ast::Pattern::Identifier(ident) => ast::Expr::Lambda {
                arg: ident,
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            },
            parse::ast::Pattern::Wildcard => ast::Expr::Lambda {
                arg: "__wildcard".to_owned(),
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            },
            _ => todo!(),
        },
        parse::ast::Expr::BinOp { op, lhs, rhs } => {
            let lhs = Box::new(canonicalize_expression(*lhs, type_defs));
            let rhs = Box::new(canonicalize_expression(*rhs, type_defs));

            match op {
                // TODO: find a point free way to do this
                parse::ast::Operator::Compose => ast::Expr::Lambda {
                    arg: "x".to_owned(),
                    body: Box::new(ast::Expr::Ap {
                        function: lhs,
                        arg: Box::new(ast::Expr::Ap {
                            function: rhs,
                            arg: Box::new(ast::Expr::Binding("x".to_owned())),
                        }),
                    }),
                },
                parse::ast::Operator::ComposeRev => ast::Expr::Lambda {
                    arg: "x".to_owned(),
                    body: Box::new(ast::Expr::Ap {
                        function: rhs,
                        arg: Box::new(ast::Expr::Ap {
                            function: lhs,
                            arg: Box::new(ast::Expr::Binding("x".to_owned())),
                        }),
                    }),
                },
                parse::ast::Operator::Pipe => ast::Expr::Ap {
                    function: lhs,
                    arg: rhs,
                },
                parse::ast::Operator::PipeRev => ast::Expr::Ap {
                    function: rhs,
                    arg: lhs,
                },
                parse::ast::Operator::Cons => ast::Expr::Ap {
                    function: Box::new(ast::Expr::Ap {
                        function: Box::new(ast::Expr::Constructor { tag: 1, arity: 2 }),
                        arg: lhs,
                    }),
                    arg: rhs,
                },

                parse::ast::Operator::Or => ast::Expr::Op {
                    op: ast::Operator::Or,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::And => ast::Expr::Op {
                    op: ast::Operator::And,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Eq => ast::Expr::Op {
                    op: ast::Operator::Eq,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Neq => ast::Expr::Op {
                    op: ast::Operator::Neq,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::LT => ast::Expr::Op {
                    op: ast::Operator::LT,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::LTE => ast::Expr::Op {
                    op: ast::Operator::LTE,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::GT => ast::Expr::Op {
                    op: ast::Operator::GT,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::GTE => ast::Expr::Op {
                    op: ast::Operator::GTE,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Concat => ast::Expr::Op {
                    op: ast::Operator::Concat,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Plus => ast::Expr::Op {
                    op: ast::Operator::Plus,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Minus => ast::Expr::Op {
                    op: ast::Operator::Minus,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Times => ast::Expr::Op {
                    op: ast::Operator::Times,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Divide => ast::Expr::Op {
                    op: ast::Operator::Divide,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Mod => ast::Expr::Op {
                    op: ast::Operator::Mod,
                    lhs,
                    rhs,
                },
                parse::ast::Operator::Power => ast::Expr::Op {
                    op: ast::Operator::Power,
                    lhs,
                    rhs,
                },
            }
        }
        parse::ast::Expr::When(expr, items) => ast::Expr::When {
            expr: Box::new(canonicalize_expression(*expr, type_defs)),
            alternatives: items
                .into_iter()
                .enumerate()
                .map(|(i, (pattern, body))| match pattern {
                    parse::ast::Pattern::Wildcard => ast::Alternative {
                        tag: 0,
                        args: vec!["_".to_owned()],
                        body: canonicalize_expression(body, type_defs),
                    },
                    parse::ast::Pattern::Identifier(ident) => ast::Alternative {
                        tag: 0,
                        args: vec![ident],
                        body: canonicalize_expression(body, type_defs),
                    },
                    parse::ast::Pattern::EmptyList => {
                        let (tag, _) = canonicalize_constructor("Empty", type_defs);
                        ast::Alternative {
                            tag,
                            args: vec![],
                            body: canonicalize_expression(body, type_defs),
                        }
                    }
                    parse::ast::Pattern::Constructor(name, sub_patterns) => {
                        let (tag, arity) = canonicalize_constructor(&name, type_defs);
                        let args = sub_patterns
                            .into_iter()
                            .enumerate()
                            .filter_map(|(i, sub_pattern)| match sub_pattern {
                                parse::ast::Pattern::Identifier(ident) => Some(ident.to_owned()),
                                parse::ast::Pattern::Wildcard => {
                                    Some("__arg".to_owned() + &i.to_string())
                                }
                                _ => None,
                            })
                            .collect::<Vec<String>>();
                        ast::Alternative {
                            tag,
                            args,
                            body: canonicalize_expression(body.clone(), type_defs),
                        }
                    }
                    parse::ast::Pattern::Tuple(patterns) => todo!(),
                })
                .collect(),
        },
        parse::ast::Expr::Unit => todo!(),
        parse::ast::Expr::Bool(_) => todo!(),
        parse::ast::Expr::Nat(nat) => ast::Expr::Num(nat as f64),
        parse::ast::Expr::Int(int) => ast::Expr::Num(int as f64),
        parse::ast::Expr::Float(float) => ast::Expr::Num(float as f64),
        parse::ast::Expr::String(string) => ast::Expr::String(string),
        parse::ast::Expr::Record(hash_map) => todo!(),
        parse::ast::Expr::Access(module, member) => ast::Expr::ModuleAccess {
            module: module,
            member: member,
        },
        parse::ast::Expr::List(exprs) => exprs.into_iter().rev().fold(
            {
                let (tag, arity) = canonicalize_constructor("Empty", type_defs);
                ast::Expr::Constructor { tag, arity }
            },
            |list, expr| ast::Expr::Ap {
                function: Box::new(ast::Expr::Ap {
                    function: {
                        let (tag, arity) = canonicalize_constructor("Cons", type_defs);
                        Box::new(ast::Expr::Constructor { tag, arity })
                    },
                    arg: Box::new(canonicalize_expression(expr, type_defs)),
                }),
                arg: Box::new(list),
            },
        ),
        // TODO: represent lists as arrays
        // ast::Expr::List(
        //     exprs
        //         .iter()
        //         .map(|expr| canonicalize_expression(expr.clone(), type_defs))
        //         .collect(),
        // ),
        parse::ast::Expr::Constructor(name) => {
            let (tag, arity) = canonicalize_constructor(&name, type_defs);
            ast::Expr::Constructor { tag, arity }
        }
        parse::ast::Expr::Tuple(exprs) => exprs.into_iter().fold(
            ast::Expr::Constructor { tag: 0, arity: 0 },
            |function, expr| ast::Expr::Ap {
                function: Box::new(function),
                arg: Box::new(canonicalize_expression(expr, type_defs)),
            },
        ),
    }
}

fn canonicalize_constructor(name: &str, type_defs: &[(String, parse::ast::TypeDef)]) -> (u8, u8) {
    type_defs
        .iter()
        .filter_map(|(_, type_def)| match type_def {
            parse::ast::TypeDef::Internal { constructors, .. } => {
                let i = constructors.iter().position(|c| &c.name == name)?;
                Some((i as u8, constructors.get(i)?.args.len() as u8))
            }
            parse::ast::TypeDef::External(_) => None,
        })
        .next()
        .expect("name should resolve due to type checking")
}

pub fn canonicalize_module(
    parse_module: parse::ast::Module,
    type_defs: &[(String, parse::ast::TypeDef)],
) -> ast::Module {
    ast::Module {
        name: parse_module.name.to_owned(),
        imports: parse_module
            .imports
            .iter()
            .map(|parse::ast::Import { module, .. }| ast::Import {
                name: module.clone(),
            })
            .collect(),
        interface: parse_module
            .interface
            .into_iter()
            .map(|export| to_camel_case(&export))
            .collect::<Vec<String>>(),
        defs: parse_module
            .defs
            .iter()
            .map(|(binding, body)| {
                (
                    to_camel_case(&binding),
                    canonicalize_expression(body.clone(), type_defs),
                )
            })
            .collect(),
    }
}

// take the parsed AST and turn it into the core language
pub fn canonicalize(mut parse_modules: Vec<parse::ast::Module>) -> Vec<ast::Module> {
    let mut type_defs = vec![];
    for parse_module in parse_modules.iter_mut() {
        type_defs.append(&mut parse_module.type_defs);
    }

    let mut core_modules = vec![];
    for parse_module in parse_modules.into_iter() {
        core_modules.push(canonicalize_module(parse_module, &type_defs));
    }
    core_modules
}
