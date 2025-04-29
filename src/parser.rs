use crate::expr::{ Expr, Lit };
use peg::{ self, error::ParseError, str::LineCol };
use unicode_ident::{ is_xid_start, is_xid_continue };
use f128::f128;


struct StringTerminator(!);
impl StringTerminator {
    const IDENTIFIER : char = '`';
    const NORMAL     : char = '"';
}


pub fn parse(script : &str) -> Result<Vec<Expr>, ParseError<LineCol>> {
    sisyphys_parser::script(script)
}


peg::parser! { grammar sisyphys_parser() for str {

    pub(super) rule script() -> Vec<Expr>
        = _ e:( e:expr() _ { e } ) ** ( "\n" _ ) { e }

    rule expr() -> Expr
        = "print" __ a:expr_args(1) { destructure_expr_args!( a => v,    ); Expr::Print (Box::new(v)) }
        / "+"     __ a:expr_args(2) { destructure_expr_args!( a => l, r, ); Expr::Add   (Box::new((l, r,))) }
        / "-"     __ a:expr_args(2) { destructure_expr_args!( a => l, r, ); Expr::Sub   (Box::new((l, r,))) }
        / "*"     __ a:expr_args(2) { destructure_expr_args!( a => l, r, ); Expr::Mul   (Box::new((l, r,))) }
        / "/"     __ a:expr_args(2) { destructure_expr_args!( a => l, r, ); Expr::Div   (Box::new((l, r,))) }
        / l:lit() { Expr::Lit(l) }

    rule expr_args(n : usize) -> Vec<Expr>
        = a:( a:expr() { a } )**<{n}> __ { a }

    rule lit() -> Lit
        = s:lit_string(StringTerminator::NORMAL) { Lit::String(s) }
        / b:lit_bool() { Lit::Bool(b) }
        / i:lit_int() { Lit::Int(i) }
        / f:lit_float() { Lit::Float(f) }

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
        = (" " / "\t")+ { () }
    rule _() -> ()
        = (" " / "\t")* { () }

} }



macro destructure_expr_args( $a:expr => $( $out:pat ),+ $(,)? ) {
    let mut a = ($a).into_iter();
    $( let $out = a.next().unwrap(); )+
}
