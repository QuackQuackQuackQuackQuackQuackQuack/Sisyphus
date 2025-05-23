use crate::expr::{ Expr, Lit };
use peg;
use unicode_ident::{ is_xid_start, is_xid_continue };
use f128::f128;


mod error;
pub use error::*;


struct StringTerminator(!);
impl StringTerminator {
    const IDENTIFIER : char = '`';
    const NORMAL     : char = '"';
}


pub fn parse<'l>(script : &'l str) -> Result<Vec<Expr>, ParserError<'l>> {
    sisyphys_parser::script(&script).map_err(|e| ParserError::from_peg(script.lines().nth(e.location.line - 1).unwrap_or(""), e))
}


peg::parser! { grammar sisyphys_parser() for str {

    pub(super) rule script() -> Vec<Expr>
        = _ e:( e:expr() _ { e } ) ** ( "\n" _ ) { e }

    rule expr() -> Expr
        = "print"   __ a:expr_args(1) { destructure_expr_args!( a => v,       ); Expr::Print         (Box::new(v)) }
        / "+"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Add           (Box::new((l, r,))) }
        / "-"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Sub           (Box::new((l, r,))) }
        / "*"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Mul           (Box::new((l, r,))) }
        / "/"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Div           (Box::new((l, r,))) }
        / "!"       __ a:expr_args(1) { destructure_expr_args!( a => b,       ); Expr::Not           (Box::new((b, ))) }
        / ">="      __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::GreaterEquals (Box::new((l, r,))) }
        / "<="      __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::LessEquals    (Box::new((l, r,))) }
        / "="       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Equals        (Box::new((l, r,))) }
        / ">"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Greater       (Box::new((l, r,))) }
        / "<"       __ a:expr_args(2) { destructure_expr_args!( a => l, r,    ); Expr::Less          (Box::new((l, r,))) }
        / "gets"    __ a:expr_args(3) { destructure_expr_args!( a => q, s, e  ); Expr::Gets          (Box::new((q, s, e))) }
        / "get"     __ a:expr_args(2) { destructure_expr_args!( a => q, i,    ); Expr::Get           (Box::new((q, i,))) }
        / "pushes"  __ a:expr_args(2) { destructure_expr_args!( a => q, l,    ); Expr::Pushes        (Box::new((q, l,))) }
        / "push"    __ a:expr_args(2) { destructure_expr_args!( a => q, v,    ); Expr::Push          (Box::new((q, v,))) }
        / "inserts" __ a:expr_args(3) { destructure_expr_args!( a => q, s, l, ); Expr::Inserts       (Box::new((q, s, l,))) }
        / "insert"  __ a:expr_args(3) { destructure_expr_args!( a => q, i, v, ); Expr::Insert        (Box::new((q, i, v,))) }
        / "sets"    __ a:expr_args(3) { destructure_expr_args!( a => q, s, l, ); Expr::Sets          (Box::new((q, s, l,))) }
        / "set"     __ a:expr_args(3) { destructure_expr_args!( a => q, i, v, ); Expr::Set           (Box::new((q, i, v,))) }
        / "len"     __ a:expr_args(1) { destructure_expr_args!( a => q,       ); Expr::Print         (Box::new(q)) }
        / "fsread"  __ a:expr_args(1) { destructure_expr_args!( a => f,       ); Expr::FSRead        (Box::new(f)) }
        / "if"      __ a:expr_args(3) { destructure_expr_args!( a => c, t, f, ); Expr::If            (Box::new((c, t, f))) }
        / "range"   __ a:expr_args(2) { destructure_expr_args!( a => i0, i1,  ); Expr::Range         (Box::new((i0, i1,))) }
        / "str"     __ a:expr_args(1) { destructure_expr_args!( a => v,       ); Expr::Str           (Box::new(v)) }
        / "int"     __ a:expr_args(1) { destructure_expr_args!( a => v,       ); Expr::Int           (Box::new(v)) }
        / l:lit() { Expr::Lit(l) }

    rule expr_args(n : usize) -> Vec<Expr>
        = a:( a:expr() { a } )**<{n}> __ { a }

    rule lit() -> Lit
        = s:lit_string(StringTerminator::NORMAL) { Lit::String(s) }
        / b:lit_bool() { Lit::Bool(b) }
        / i:lit_int() { Lit::Int(i) }
        / f:lit_float() { Lit::Float(f) }
        / "queue" { Lit::ExprQueue }

    rule lit_bool() -> bool
        = "true" { true } 
        / "false" { false }

    rule lit_float() -> f128
        = quiet!{ f:$(lit_int() "." lit_int()) {? f128::parse(f).or(Err("bad float")) } }
        / expected!("float")

    rule lit_int() -> i128
        = quiet!{ s:$(['0'..='9']+) {? s.parse().or(Err("bad int")) } }
        / expected!("integer")

    rule ident() -> String
        = quiet!{
            start_char:[c if is_xid_start(c)] continue_chars:([c if is_xid_continue(c)])* {
            let mut identifier = String::with_capacity(4 * (1 + continue_chars.len()));
            identifier.push(start_char);
            for ch in continue_chars { identifier.push(ch); }
            return identifier;
        }}
        / lit_string(StringTerminator::IDENTIFIER)
        / expected!("ident")

    rule lit_string(terminator : char) -> String
        = quiet!{ [ c if c == terminator ] }
            s:(lit_string_inner(terminator)*)
            quiet!{ [ c if c == terminator ] }
            { s.into_iter().collect() }
        / expected!("string")
    rule lit_string_inner(terminator : char) -> char
        = !([ c if c == terminator ] / "\r" / "\n" / "\\") c:[_] { c }
        / quiet!{ "\\\\" } { '\\' }
        / quiet!{ "\\" [ c if c == terminator ] } { terminator }
        / quiet!{ "\\\"" } { '"' }
        / quiet!{ "\\'" } { '\'' }
        / quiet!{ "\\`" } { '`' }
        / quiet!{ "\\0" } { '\0' }
        / quiet!{ "\\n" } { '\n' }
        / quiet!{ "\\r" } { '\r' }
        / quiet!{ "\\t" } { '\t' }
        / quiet!{ "\\x" } unicode_code_point:(u:$(
            (quiet!{ [ '0'..='9' | 'a'..='f' | 'A'..='F' ] } / expected!("valid hexadecimal code point"))*<2>
        )) {?
            return u8::from_str_radix(unicode_code_point, 16)
                .ok().and_then(|ucp| char::from_u32(ucp as u32))
                .ok_or("valid hexadecimal code point");
        }
        / quiet!{ "\\u" } "{" unicode_code_point:$(
            (quiet!{ [ '0'..='9' | 'a'..='f' | 'A'..='F' ] } / expected!("valid unicode code point"))+
        ) "}" {?
            return u32::from_str_radix(unicode_code_point, 16)
                .ok().and_then(char::from_u32)
                .ok_or("valid unicode code point");
        }
        / expected!("valid escape sequence")


    rule __() -> ()
        = quiet!{ (" " / "\t")+ { () } }
    rule _() -> ()
        = quiet!{ (" " / "\t")* { () } }

} }



macro destructure_expr_args( $a:expr => $( $out:pat ),+ $(,)? ) {
    let mut a = ($a).into_iter();
    $( let $out = a.next().unwrap(); )+
}
