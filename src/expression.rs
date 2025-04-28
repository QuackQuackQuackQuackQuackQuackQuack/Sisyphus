pub enum Expr {
    Lit(Lit)
}

pub enum Lit {
    Bool(bool),
    Int(i128),
    Float(f128),
    Char(char),
    String(String)
}
