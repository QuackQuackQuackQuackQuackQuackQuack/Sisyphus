use f128::f128;


#[derive(Debug)]
pub enum Expr {
    Print(Box<Expr>), // message
    Add(Box<(Expr, Expr,)>), // left, right
    Sub(Box<(Expr, Expr,)>), // left, right
    Mul(Box<(Expr, Expr,)>), // left, right
    Div(Box<(Expr, Expr,)>), // left, right
    Get(Box<(Expr, Expr,)>), // iterable, index
    Gets(Box<(Expr, Expr, Expr)>), // iterable, start index, end index
    Push(Box<(Expr, Expr,)>), // iterable, entry
    Pushes(Box<(Expr, Expr)>), // iterable, iterable
    Insert(Box<(Expr, Expr, Expr,)>), // iterable, index, entry
    Inserts(Box<(Expr, Expr, Expr)>), // iterable, start index, iterable
    Set(Box<(Expr, Expr, Expr)>), // iterable, index, entry
    Sets(Box<(Expr, Expr, Expr)>), // iterable, start index, iterable
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
