use crate::expr::Expr;
use std::collections::VecDeque;


mod expr;
pub use expr::*;

mod value;
pub use value::*;


pub struct Executor {
    exprs : VecDeque<Expr>
}

impl Executor {
    pub fn new() -> Self {
        Self {
            exprs : VecDeque::new()
        }
    }
}

impl Executor {
    pub fn tick(&mut self) -> bool {
        let Some(expr) = self.exprs.pop_front()
            else { return false; };
        let _ = expr.execute(self);
        true
    }
}

impl Executor {

    pub fn push_exprs<I>(&mut self, exprs : I)
    where
        I : IntoIterator<Item = Expr>
    {
        self.exprs.extend(exprs);
    }

}
