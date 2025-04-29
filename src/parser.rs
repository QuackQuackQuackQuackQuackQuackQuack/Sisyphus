use crate::expr::{ Expr, Lit };
use peg;


peg::parser! { grammar sisyphys_parser() for str {

    rule args(count : usize) -> Vec<Expr>
        = { todo!() }

    rule lit() -> Lit
        = { todo!() }

    rule __() -> ()
        = (" " / "\t")+ { () }
    rule _() -> ()
        = (" " / "\t")* { () }

} }
