use core::fmt;
use f128::f128;


#[derive(Debug, Clone)]
pub enum Expr {
    Print(Box<Expr>), // message
    Add(Box<(Expr, Expr,)>), // left, right
    Sub(Box<(Expr, Expr,)>), // left, right
    Mul(Box<(Expr, Expr,)>), // left, right
    Div(Box<(Expr, Expr,)>), // left, right
    Not(Box<Expr>), 
    Equals(Box<(Expr, Expr,)>),
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
    If(Box<(Expr, Expr, Expr)>), // conditional. runs second expr if the first is true, otherwise runs the third
    Range(Box<(Expr, Expr)>), // range from first to second, inclusive of first but not second
    Str(Box<Expr>),
    Int(Box<Expr>),
    Lit(Lit), // value
}

impl fmt::Display for Expr {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Expr::Print   (expr) => write!(f, "print {}", expr),
            Expr::Add     (expr) => write!(f, "+ {} {}", expr.0, expr.1),
            Expr::Sub     (expr) => write!(f, "- {} {}", expr.0, expr.1),
            Expr::Mul     (expr) => write!(f, "* {} {}", expr.0, expr.1),
            Expr::Div     (expr) => write!(f, "/ {} {}", expr.0, expr.1),
            Expr::Get     (expr) => write!(f, "get {} {}", expr.0, expr.1),
            Expr::Gets    (expr) => write!(f, "gets {} {} {}", expr.0, expr.1, expr.2),
            Expr::Push    (expr) => write!(f, "push {} {}", expr.0, expr.1),
            Expr::Pushes  (expr) => write!(f, "pushes {} {}", expr.0, expr.1),
            Expr::Insert  (expr) => write!(f, "insert {} {} {}", expr.0, expr.1, expr.2),
            Expr::Inserts (expr) => write!(f, "inserts {} {} {}", expr.0, expr.1, expr.2),
            Expr::Set     (expr) => write!(f, "set {} {} {}", expr.0, expr.1, expr.2),
            Expr::Sets    (expr) => write!(f, "sets {} {} {}", expr.0, expr.1, expr.2),
            Expr::Len     (expr) => write!(f, "len {}", expr),
            Expr::FSRead  (expr) => write!(f, "fsread {}", expr),
            Expr::Lit     (lit)  => write!(f, "{}", lit),
            Expr::If      (expr) => write!(f, "if {} {} {}", expr.0, expr.1, expr.2),
            Expr::Range   (expr) => write!(f, "range {} {}", expr.0, expr.1),
            Expr::Str     (expr) => write!(f, "str {}", expr),
            Expr::Int     (expr) => write!(f, "int {}", expr),
            Expr::Not     (expr) => write!(f, "! {}", expr),
            Expr::Equals  (expr) => write!(f, "{} = {}", expr.0, expr.1),
        }
    }
}


#[derive(Debug, Clone)]
pub enum Lit {
    Bool(bool),
    Int(i128),
    Float(f128),
    String(String),
    ExprQueue
}

impl fmt::Display for Lit {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Lit::Bool      (v) => if (*v) { write!(f, "true") } else { write!(f, "false") },
            Lit::Int       (v) => write!(f, "{}", v),
            Lit::Float     (v) => write!(f, "{}", v),
            Lit::String    (v) => write!(f, "{:?}", v),
            Lit::ExprQueue     => write!(f, "queue")
        }
    }
}
