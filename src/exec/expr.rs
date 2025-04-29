use crate::expr::{ Expr, Lit };
use crate::exec::Value;


pub trait Execute {
    fn execute(self) -> Value;
}


impl Execute for Expr {
    fn execute(self) -> Value {
        match (self) {
            Self::Print(expr) => { println!("{}", expr.execute()); Value::Unit },
            Self::Add(args) => args.0.execute() + args.1.execute(),
            Self::Sub(args) => args.0.execute() - args.1.execute(),
            Self::Mul(args) => args.0.execute() * args.1.execute(),
            Self::Div(args) => todo!(),
            Self::Lit(lit) => lit.execute(),
        }
    }
}

impl Execute for Lit {
    fn execute(self) -> Value {
        match (self) {
            Self::Bool   (v) => Value::Bool(v),
            Self::Int    (v) => Value::Int(v),
            Self::Float  (v) => Value::Float(v),
            Self::String (v) => Value::String(v),
        }
    }
}
