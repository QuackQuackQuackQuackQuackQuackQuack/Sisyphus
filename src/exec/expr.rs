use crate::expr::{ Expr, Lit };
use crate::exec::{ Executor, Value };


pub trait Execute {
    fn execute(self, e : &mut Executor) -> Value;
}


impl Execute for Expr {
    fn execute(self, e : &mut Executor) -> Value {
        match (self) {
            Self::Print(expr) => { println!("{}", expr.execute(e)); Value::Unit },
            Self::Add(args) => args.0.execute(e) + args.1.execute(e),
            Self::Sub(args) => args.0.execute(e) - args.1.execute(e),
            Self::Mul(args) => args.0.execute(e) * args.1.execute(e),
            Self::Div(args) => args.0.execute(e) / args.1.execute(e),
            Self::Get(args) => todo!(),
            Self::Gets(args) => todo!(),
            Self::Push(args) => todo!(),
            Self::Pushes(args) => todo!(),
            Self::Insert(args) => todo!(),
            Self::Inserts(args) => todo!(),
            Self::Set(args) => todo!(),
            Self::Sets(args) => todo!(),
            Self::Len(args) => todo!(),
            Self::FSRead(args) => todo!(),
            Self::Lit(lit) => lit.execute(e),
            Self::If(args) => todo!(),
            Self::Range(args) => todo!(),
        }
    }
}

impl Execute for Lit {
    fn execute(self, _e : &mut Executor) -> Value {
        match (self) {
            Self::Bool      (v) => Value::Bool(v),
            Self::Int       (v) => Value::Int(v),
            Self::Float     (v) => Value::Float(v),
            Self::String    (v) => Value::String(v),
            Self::ExprQueue     => Value::ExprQueue
        }
    }
}
