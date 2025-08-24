use crate::{
    ast::source::{Expr, Expr_, Operator, Pattern, Pattern_, Type, Type_},
    util::indent,
};

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Type {
    fn pretty_print(&self) -> String {
        match &self.inner {
            Type_::Unit => "()".to_owned(),
            Type_::Identifier(ident) => ident.to_owned(),
            Type_::Fn(arg, ret) => {
                "(".to_owned() + &arg.pretty_print() + " -> " + &ret.pretty_print() + ")"
            }
            Type_::Record(fields) => {
                "{".to_owned()
                    + &fields
                        .iter()
                        .map(|(name, value)| name.to_owned() + ": " + &value.pretty_print())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + "}"
            }
            Type_::Tuple(first, second, rest) => {
                let rest = if rest.is_empty() {
                    ""
                } else {
                    &(",".to_owned()
                        + &rest
                            .iter()
                            .map(|t| t.pretty_print())
                            .collect::<Vec<_>>()
                            .join(", "))
                };
                "(".to_owned() + &first.pretty_print() + "," + &second.pretty_print() + rest + ")"
            }
            Type_::Constructor(cons, first_arg, args) => {
                cons.pretty_print()
                    + &first_arg.pretty_print()
                    + &args
                        .iter()
                        .map(|arg| arg.pretty_print())
                        .collect::<Vec<String>>()
                        .join(" ")
            }
            Type_::Variable(name) => name.to_owned(),
            Type_::QualifiedIdentifier(module_name, name) => module_name.0.join(".") + "." + &name,
        }
    }
}

impl PrettyPrint for Operator {
    fn pretty_print(&self) -> String {
        use Operator::*;
        match self {
            Compose => "<<",
            ComposeRev => ">>",
            Pipe => "<|",
            PipeRev => "|>",
            Or => "||",
            And => "&&",
            Eq => "==",
            Neq => "!=",
            LT => "<",
            LTE => "<=",
            GT => ">",
            GTE => ">=",
            Cons => "::",
            Concat => "++",
            Plus => "+",
            Minus => "-",
            Times => "*",
            Divide => "/",
            Mod => "%",
            Power => "^",
        }
        .to_string()
    }
}

impl PrettyPrint for Pattern {
    fn pretty_print(&self) -> String {
        match &self.inner {
            Pattern_::Wildcard => "_".to_string(),
            Pattern_::Identifier(ident) => ident.to_string(),
            Pattern_::Constructor(tag, patterns) => match &**tag {
                "Empty" => "[]".to_owned(),
                // TODO: cons
                _ => {
                    tag.to_owned()
                        + &patterns
                            .iter()
                            .map(|pattern| pattern.pretty_print())
                            .collect::<Vec<String>>()
                            .join(" ")
                }
            },
            Pattern_::Tuple(patterns) => {
                "(".to_owned()
                    + &patterns
                        .iter()
                        .map(|pattern| pattern.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + ")"
            }
            Pattern_::Cons(element, list) => element.pretty_print() + "::" + &list.pretty_print(),
        }
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self) -> String {
        match &self.inner {
            Expr_::Identifier(i) => i.to_owned(),
            Expr_::BinOp { op, lhs, rhs } => {
                "(".to_owned()
                    + &lhs.pretty_print()
                    + " "
                    + &op.pretty_print()
                    + " "
                    + &rhs.pretty_print()
                    + ")"
            }
            Expr_::Int(i) => i.to_string(),
            Expr_::Float(x) => x.to_string(),
            Expr_::String(s) => "\"".to_owned() + s + "\"",
            Expr_::Record(fields) => {
                "{ ".to_owned()
                    + &fields
                        .iter()
                        .map(|(field, value)| field.to_owned() + " : " + &value.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + " }"
            }
            Expr_::QualifiedIdentifier(module, member) => module.0.join(".") + "." + &member,
            Expr_::QualifiedConstructor(module, member) => module.0.join(".") + "." + &member,
            Expr_::List(xs) => {
                "[".to_string()
                    + &xs
                        .iter()
                        .map(|x| x.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + "]"
            }
            Expr_::Ap(f, xs) => {
                "(".to_string() + &f.pretty_print() + " " + &xs.pretty_print() + ")"
            }
            Expr_::Lambda(arg, body) => {
                "(\\".to_owned() + &arg.pretty_print() + " -> " + &body.pretty_print() + ")"
            }
            Expr_::When(expr, branch, branches) => {
                "when ".to_string()
                    + &expr.pretty_print()
                    + " is\n"
                    + &indent(
                        &("| ".to_string()
                            + &branch.0.pretty_print()
                            + " -> "
                            + &branch.1.pretty_print()),
                    )
                    + &indent(
                        &("| ".to_string()
                            + &branches
                                .iter()
                                .map(|(pat, body)| {
                                    pat.pretty_print() + " -> " + &body.pretty_print()
                                })
                                .collect::<Vec<String>>()
                                .join("\n| ")),
                    )
            }
            Expr_::Unit => "()".to_string(),
            Expr_::If(cond, then_branch, else_branch) => {
                "(if ".to_string()
                    + &cond.pretty_print()
                    + " then "
                    + &then_branch.pretty_print()
                    + " else "
                    + &else_branch.pretty_print()
                    + ")"
            }
            Expr_::Bool(bool) => bool.to_string(),
            Expr_::External(name) => "@".to_string() + name,
            Expr_::Let(pattern, def, body) => {
                "(let ".to_owned()
                    + &pattern.pretty_print()
                    + " = "
                    + &def.pretty_print()
                    + "; "
                    + &body.pretty_print()
            }
            Expr_::Bind(pattern, def, body) => {
                "(let ".to_owned()
                    + &pattern.pretty_print()
                    + " <- "
                    + &def.pretty_print()
                    + "; "
                    + &body.pretty_print()
            }
            Expr_::Constructor(cons) => cons.to_owned(),
            Expr_::Tuple(exprs) => {
                "(".to_owned()
                    + &exprs
                        .iter()
                        .map(|expr| expr.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + ")"
            }
        }
    }
}
