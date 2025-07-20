use crate::core::ast::*;
use std::fs::File;
use std::io::Write;

trait ToJs {
    fn to_js(&self) -> String;
}

impl ToJs for Term {
    fn to_js(&self) -> String {
        match self {
            Term::String(s) => "\"".to_owned() + s + "\"",
            Term::Num(num) => num.to_string(),
            Term::Binding(binding) => binding.to_owned(),
            Term::Constructor { tag, arity } => format!("Pack{{{}, {}}}", tag.to_string(), arity.to_string()),
        }
    }
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
        Expr::Term(term) => term.to_js(),
        Expr::String(s) => s.clone(),
        Expr::Lambda { arg, body } => "(".to_owned() + &arg + ") => " + &body.to_js(),
        Expr::Op { op, lhs, rhs } => {
                    "(".to_owned() + &lhs.to_js() + " " + &op.to_js() + " " + &rhs.to_js() + ")"
                },
        Expr::When { expr, alternatives } => {
                    "(() => {
                    const expr = ".to_owned() + &expr.to_js() + ";
                    const f = (x) => x;
                    return f(expr)
                })()"
                },
        Expr::Ap {function, arg} => "(".to_owned() + &function.to_js() + ")(" + &arg.to_js() + ")",
        Expr::List(elements) =>
                    "[".to_owned() + &elements.iter().map(ToJs::to_js).collect::<Vec<String>>().join(",") + "]",
        Expr::Let { defs, body } => todo!(),
        Expr::LetRec { defs, body } => todo!(),
    }
    }
}


fn compile_module(module : Module) ->Result<(), Box<dyn std::error::Error>> {
    println!("\n\ncompiling {}.lang", module.name);
    let mut file = File::create(&("build/".to_owned() + &module.name + ".js"))?;
    for (binding, body) in &module.defs {
        let js = "const ".to_owned() + &binding + " = " + &body.to_js() + ";\n";
        file.write(js.as_bytes()).expect("fuck it just crash when this doesn't work");
        print!("{}", js);
    }

    Ok (())
}
