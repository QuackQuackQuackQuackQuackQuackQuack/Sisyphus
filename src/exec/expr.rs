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
            Self::Get(args) => Self::exec_get(e, args.0.execute(e), args.1.execute(e)),
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
impl Expr {

    fn exec_get(e : &mut Executor, q : Value, i : Value) -> Value {
        let Value::Int(i) = i
            else { return Value::Error; };
        if (i < 0) { return Value::Error; }
        let i = i as usize;
        match (q) {
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::String    (v) => v.chars().nth(i).map_or(Value::Error, |ch| Value::String(ch.to_string())),
            Value::Error         => Value::Error,
            Value::ExprQueue     => e.exprs.get(i).map_or(Value::Error, |v| Value::String(v.to_string())),
            Value::Array     (q) => q.get(i).map_or(Value::Error, |v| v.clone())
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
