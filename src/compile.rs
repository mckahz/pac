use crate::core::ast::*;
use std::fs::File;
use std::io::Write;

trait ToJs {
    fn to_js(&self) -> String;
}

impl ToJs for Operator {
    fn to_js(&self) -> String {
        let s = match self {
            Operator::Or => "||",
            Operator::And => "&&",
            Operator::Eq => "===",
            Operator::Neq => "!=",
            Operator::LT => "<",
            Operator::LTE => "<=",
            Operator::GT => ">",
            Operator::GTE => ">=",
            Operator::Concat => "+",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Divide => "/",
            Operator::Mod => "%",
            Operator::Power => "^",
        };
        s.to_owned()
    }
}

impl ToJs for Expr {
    fn to_js(&self) -> String {
        match self {
            Expr::String(s) => "\"".to_owned() + s + "\"",
            Expr::Num(num) => num.to_string(),
            Expr::Binding(binding) => binding.to_owned(),
            Expr::Constructor { tag, arity } => {
                format!("__pack({}, {})", tag.to_string(), arity.to_string())
            }
            Expr::List(exprs) => {
                "[".to_owned()
                    + &exprs
                        .iter()
                        .map(|expr| expr.to_js())
                        .collect::<Vec<String>>()
                        .join("")
                    + "]"
            }
            Expr::Extern(string) => match &string[..] {
                "println" => "console.log".to_owned(),
                "crash" => "console.error".to_owned(),
                _ => string.to_owned(),
            },
            Expr::Lambda { arg, body } => "(".to_owned() + &arg + ") => " + &body.to_js(),
            Expr::Op { op, lhs, rhs } => {
                "(".to_owned() + &lhs.to_js() + " " + &op.to_js() + " " + &rhs.to_js() + ")"
            }
            Expr::When { expr, alternatives } => {
                "(() => {
                    const __expr = "
                    .to_owned()
                    + &expr.to_js()
                    + ";
                    const __function = ["
                    + &alternatives
                        .iter()
                        .map(|Alternative { tag, args, body }| {
                            "(".to_owned() + &args.join(",") + ") => " + &body.to_js()
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                    + "][__expr.tag];
                    return __function(... __expr.args);
                })()"
            }
            Expr::Ap { function, arg } => function.to_js() + "(" + &arg.to_js() + ")",
            Expr::Let { defs, body } => todo!(),
            Expr::LetRec { defs, body } => todo!(),
        }
    }
}

pub fn compile_module(module: Module) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&("build/".to_owned() + &module.name + ".js"))?;
    for (binding, body) in &module.defs {
        let js = "const ".to_owned() + &binding + " = " + &body.to_js() + ";\n\n";
        file.write(js.as_bytes())
            .expect("fuck it just crash when this doesn't work");
    }

    Ok(())
}
