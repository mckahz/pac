use nom_supreme::error::ErrorTree;

use crate::parse::ast::{Expr, Import, Operator, Pattern, Type};
use crate::util::indent;

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
        match self {
            Type::Unit => "()".to_owned(),
            Type::Int => "Int".to_owned(),
            Type::Nat => "Nat".to_owned(),
            Type::Float => "Float".to_owned(),
            Type::String => "String".to_owned(),
            Type::Identifier(ident) => ident.to_owned(),
            Type::Fn(arg, ret) => {
                "(".to_owned() + &arg.pretty_print() + " -> " + &ret.pretty_print() + ")"
            }
            Type::Record(fields) => {
                "{".to_owned()
                    + &fields
                        .iter()
                        .map(|(name, value)| name.to_owned() + ": " + &value.pretty_print())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + "}"
            }
            Type::Tuple(types) => {
                "(".to_owned()
                    + &types
                        .iter()
                        .map(|t| t.pretty_print())
                        .collect::<Vec<_>>()
                        .join(", ")
                    + ")"
            }
            Type::Bool => "Bool".to_string(),
            Type::External(name) => "extern \"".to_string() + &name.to_string() + "\"",
            Type::Cons(name, args) => {
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
        match self {
            Pattern::Wildcard => "_".to_string(),
            Pattern::Identifier(ident) => ident.to_string(),
            Pattern::EmptyList => "[]".to_string(),
            Pattern::Constructor(tag, patterns) => {
                tag.to_owned()
                    + &patterns
                        .iter()
                        .map(|pattern| pattern.pretty_print())
                        .collect::<Vec<String>>()
                        .join(" ")
            }
            Pattern::Tuple(patterns) => {
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
        match self {
            Expr::Identifier(i) => i.to_owned(),
            Expr::BinOp { op, lhs, rhs } => {
                "(".to_owned()
                    + &lhs.pretty_print()
                    + " "
                    + &op.pretty_print()
                    + " "
                    + &rhs.pretty_print()
                    + ")"
            }
            Expr::Nat(n) => n.to_string(),
            Expr::Int(i) => i.to_string(),
            Expr::Float(x) => x.to_string(),
            Expr::String(s) => "\"".to_owned() + s + "\"",
            Expr::Record(fields) => {
                "{ ".to_owned()
                    + &fields
                        .iter()
                        .map(|(field, value)| field.to_owned() + " : " + &value.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + " }"
            }
            Expr::Access(module, member) => module.to_owned() + "." + &member,
            Expr::List(xs) => {
                "[".to_string()
                    + &xs
                        .iter()
                        .map(|x| x.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + "]"
            }
            Expr::Ap(f, xs) => "(".to_string() + &f.pretty_print() + " " + &xs.pretty_print() + ")",
            Expr::Lambda(arg, body) => {
                "(\\".to_owned() + &arg.pretty_print() + " -> " + &body.pretty_print() + ")"
            }
            Expr::When(expr, branches) => {
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
            Expr::Unit => "()".to_string(),
            Expr::If(cond, then_branch, else_branch) => {
                "(if ".to_string()
                    + &cond.pretty_print()
                    + " then "
                    + &then_branch.pretty_print()
                    + " else "
                    + &else_branch.pretty_print()
                    + ")"
            }
            Expr::Bool(bool) => bool.to_string(),
            Expr::External(name) => "@".to_string() + name,
            Expr::Let(pattern, def, body) => {
                "(let ".to_owned()
                    + &pattern.pretty_print()
                    + " = "
                    + &def.pretty_print()
                    + "; "
                    + &body.pretty_print()
            }
            Expr::Bind(pattern, def, body) => {
                "(let ".to_owned()
                    + &pattern.pretty_print()
                    + " <- "
                    + &def.pretty_print()
                    + "; "
                    + &body.pretty_print()
            }
            Expr::Constructor(cons) => cons.to_owned(),
            Expr::Tuple(exprs) => {
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

fn depth(error_tree: &ErrorTree<&str>) -> usize {
    use nom_supreme::error::*;
    match error_tree {
        GenericErrorTree::Base { location, kind } => location.len(),
        GenericErrorTree::Stack { base, contexts } => depth(base),
        GenericErrorTree::Alt(subtrees) => subtrees
            .iter()
            .map(|subtree| depth(subtree))
            .min()
            .unwrap_or(0),
    }
}

impl PrettyPrint for ErrorTree<&str> {
    fn pretty_print(&self) -> String {
        use nom::error::*;
        use nom_supreme::error::*;
        match self {
            GenericErrorTree::Base { location, kind } => {
                (match kind {
                    BaseErrorKind::Expected(expectation) => {
                        "expected ".to_string()
                            + &(match expectation {
                                Expectation::Tag(tag) => tag.to_string(),
                                Expectation::Char(c) => c.to_string(),
                                Expectation::Alpha => "alphabetic character".to_string(),
                                Expectation::Digit => "digit".to_string(),
                                Expectation::HexDigit => "hexidecimal digit".to_string(),
                                Expectation::OctDigit => "octal digit".to_string(),
                                Expectation::AlphaNumeric => {
                                    "alphabetic / numeric character".to_string()
                                }
                                Expectation::Space => "space".to_string(),
                                Expectation::Multispace => "multiple spaces".to_string(),
                                Expectation::CrLf => todo!(),
                                Expectation::Eof => "end of file".to_string(),
                                Expectation::Something => "something".to_string(),
                                _ => todo!(),
                            })
                    }
                    BaseErrorKind::Kind(kind) => match kind {
                        ErrorKind::Tag => "tag".to_string(),
                        ErrorKind::MapRes => todo!(),
                        ErrorKind::MapOpt => todo!(),
                        ErrorKind::Alt => todo!(),
                        ErrorKind::IsNot => todo!(),
                        ErrorKind::IsA => todo!(),
                        ErrorKind::SeparatedList => todo!(),
                        ErrorKind::SeparatedNonEmptyList => todo!(),
                        ErrorKind::Many0 => todo!(),
                        ErrorKind::Many1 => todo!(),
                        ErrorKind::ManyTill => todo!(),
                        ErrorKind::Count => todo!(),
                        ErrorKind::TakeUntil => todo!(),
                        ErrorKind::LengthValue => todo!(),
                        ErrorKind::TagClosure => todo!(),
                        ErrorKind::Alpha => todo!(),
                        ErrorKind::Digit => todo!(),
                        ErrorKind::HexDigit => todo!(),
                        ErrorKind::OctDigit => todo!(),
                        ErrorKind::AlphaNumeric => todo!(),
                        ErrorKind::Space => todo!(),
                        ErrorKind::MultiSpace => todo!(),
                        ErrorKind::LengthValueFn => todo!(),
                        ErrorKind::Eof => todo!(),
                        ErrorKind::Switch => todo!(),
                        ErrorKind::TagBits => todo!(),
                        ErrorKind::OneOf => todo!(),
                        ErrorKind::NoneOf => todo!(),
                        ErrorKind::Char => todo!(),
                        ErrorKind::CrLf => todo!(),
                        ErrorKind::RegexpMatch => todo!(),
                        ErrorKind::RegexpMatches => todo!(),
                        ErrorKind::RegexpFind => todo!(),
                        ErrorKind::RegexpCapture => todo!(),
                        ErrorKind::RegexpCaptures => todo!(),
                        ErrorKind::TakeWhile1 => todo!(),
                        ErrorKind::Complete => todo!(),
                        ErrorKind::Fix => todo!(),
                        ErrorKind::Escaped => todo!(),
                        ErrorKind::EscapedTransform => todo!(),
                        ErrorKind::NonEmpty => todo!(),
                        ErrorKind::ManyMN => todo!(),
                        ErrorKind::Not => todo!(),
                        ErrorKind::Permutation => todo!(),
                        ErrorKind::Verify => todo!(),
                        ErrorKind::TakeTill1 => todo!(),
                        ErrorKind::TakeWhileMN => todo!(),
                        ErrorKind::TooLarge => todo!(),
                        ErrorKind::Many0Count => todo!(),
                        ErrorKind::Many1Count => todo!(),
                        ErrorKind::Float => "float".to_string(),
                        ErrorKind::Satisfy => "satisfy".to_string(),
                        ErrorKind::Fail => "failure".to_string(),
                    },
                    BaseErrorKind::External(_) => todo!(),
                }) + " at\n"
                    + &indent(&location)
            }
            GenericErrorTree::Stack { base, contexts } => {
                let context = contexts
                    .iter()
                    .rev()
                    .filter_map(|(_, context)| match context {
                        StackContext::Kind(_) => None,
                        StackContext::Context(context) => Some((*context).to_string()),
                    })
                    .collect::<Vec<String>>()
                    .join("\n   ");
                format!("{}\n{}", context, indent(&base.pretty_print()))
            }
            GenericErrorTree::Alt(sub_trees) => {
                "|| ".to_string()
                    + &sub_trees
                        .iter()
                        .fold(None, |best_attempt, attempt| match best_attempt {
                            None => Some(attempt),
                            Some(sub_tree) => Some(if depth(attempt) < depth(sub_tree) {
                                attempt
                            } else {
                                sub_tree
                            }),
                        })
                        .unwrap()
                        .pretty_print()
            }
        }
    }
}
