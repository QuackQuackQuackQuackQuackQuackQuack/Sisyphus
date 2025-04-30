use f128::f128;


#[derive(Debug)]
pub enum Expr {
    Print(Box<Expr>), // message
    Add(Box<(Expr, Expr,)>), // left, right
    Sub(Box<(Expr, Expr,)>), // left, right
    Mul(Box<(Expr, Expr,)>), // left, right
    Div(Box<(Expr, Expr,)>), // left, right
    Get(Box<(Expr, Expr,)>), // iterable, index
    Push(Box<(Expr, Expr,)>), // iterable, entry
    Insert(Box<(Expr, Expr, Expr,)>), // iterable, index, entry
    Set(Box<(Expr, Expr, Expr)>), // iterable, index, entry
    Len(Box<Expr>), // iterable
    FSRead(Box<Expr>), // file name
    Lit(Lit), // value
    If(Box<(Expr, Expr, Expr)>), // conditional. runs second expr if the first is true, otherwise runs the third
    Range(Box<(Expr, Expr)>) // range from first to second, inclusive of first but not second
}

#[derive(Debug)]
pub enum Lit {
    Bool(bool),
    Int(i128),
    Float(f128),
    String(String),
    ExprQueue
}
