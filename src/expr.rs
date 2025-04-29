pub enum Expr {
    Print(Box<Expr>),
    Add(Box<(Expr, Expr,)>),
    Sub(Box<(Expr, Expr,)>),
    Mul(Box<(Expr, Expr,)>),
    Div(Box<(Expr, Expr,)>),
    Lit(Lit)
}

pub enum Lit {
    Bool(bool),
    Int(i128),
    Float(f128),
    String(String)
}
