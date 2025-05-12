use crate::expr::{ Expr, Lit };
use std::collections::VecDeque;


mod expr;
pub use expr::*;

mod value;
pub use value::*;


pub struct Executor {
    latest_expr  : Expr,
    queued_exprs : VecDeque<Expr>
}

impl Executor {
    pub fn new() -> Self {
        Self {
            latest_expr  : Expr::Lit(Lit::Bool(false)),
            queued_exprs : VecDeque::new()
        }
    }
}

impl Executor {
    pub fn tick(&mut self) -> bool {
        let Some(expr) = self.queued_exprs.pop_front()
            else { return false; };
        self.latest_expr = expr.clone();
        let _ = expr.execute(self);
        true
    }
}

impl Executor {

    pub fn get_expr(&self, index : usize) -> Option<&Expr> {
        match (index) {
            0   => Some(&self.latest_expr),
            1.. => self.queued_exprs.get(index - 1)
        }
    }

    pub fn get_exprs(&self, i0 : usize, i1 : usize) -> Option<Vec<&Expr>> {
        (i0..i1).map(|index| self.get_expr(index)).collect()
    }
    pub fn get_exprs_values(&self, i0 : usize, i1 : usize) -> Option<Vec<Value>> {
        (i0..i1).map(|index| self.get_expr(index).map(|expr| Value::String(expr.to_string()))).collect()
    }

    pub fn len_exprs(&self) -> usize {
        1 + self.queued_exprs.len()
    }

    pub fn push_exprs<I>(&mut self, exprs : I)
    where
        I : IntoIterator<Item = Expr>
    {
        self.queued_exprs.extend(exprs);
    }

    pub fn set_expr(&mut self, index : usize, expr : Expr) -> Result<(), ()> {
        match (index) {
            0   => Ok(()),
            1.. => {
                let ptr = self.queued_exprs.get_mut(index - 1).ok_or(())?;
                *ptr = expr;
                Ok(())
            }
        }
    }

    pub fn sets_expr<I>(&mut self, start_index : usize, exprs : I) -> Result<(), ()>
    where
        I : IntoIterator<Item = Expr>
    {
        for (i, expr) in exprs.into_iter().enumerate() {
            let index = start_index + i;
            self.set_expr(index, expr)?;
        }
        Ok(())
    }

    pub fn insert_expr(&mut self, index : usize, expr : Expr) -> Result<(), ()> { 
        match (index) {
            0 => {
                // don't use expr
                // push everything forward
                self.queued_exprs.push_front(self.latest_expr.clone());
                Ok(())
            },
            1.. => {
                if (index > self.queued_exprs.len()) {
                    return Err(());
                }
                self.queued_exprs.insert(index - 1, expr);
                Ok(())
            }

        }
    }
    
    pub fn inserts_expr<I>(&mut self, start_index : usize, exprs : I) -> Result<(), ()>
    where
        I : IntoIterator<Item = Expr>
    {
        for (i, expr) in exprs.into_iter().enumerate() {
            let index = start_index + i;
            self.insert_expr(index, expr)?;
        }
        Ok(())
    }
}
