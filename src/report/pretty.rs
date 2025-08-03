use crate::{
    ast::source::{Expr, Expr_, Import, Operator, Pattern, Pattern_, Type, Type_},
    util::indent,
};

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Import {
    fn pretty_print(&self) -> String {
        "import ".to_string() + &self.module + ";"
    }
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
            Type_::Tuple(types) => {
                "(".to_owned()
                    + &types
                        .iter()
                        .map(|t| t.pretty_print())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + ")"
            }
            Type_::External(name) => "extern \"".to_string() + &name.to_string() + "\"",
            Type_::Cons(name, args) => {
                name.to_owned()
                    + &args
                        .iter()
                        .map(|arg| arg.pretty_print())
                        .collect::<Vec<String>>()
                        .join(" ")
            }
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
            Pattern_::EmptyList => "[]".to_string(),
            Pattern_::Constructor(tag, patterns) => {
                tag.to_owned()
                    + &patterns
                        .iter()
                        .map(|pattern| pattern.pretty_print())
                        .collect::<Vec<String>>()
                        .join(" ")
            }
            Pattern_::Tuple(patterns) => {
                "(".to_owned()
                    + &patterns
                        .iter()
                        .map(|pattern| pattern.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + ")"
            }
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
            Expr_::Nat(n) => n.to_string(),
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
            Expr_::Access(module, member) => module.to_owned() + "." + &member,
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
            Expr_::When(expr, branches) => {
                "when ".to_string()
                    + &expr.pretty_print()
                    + " is\n"
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
