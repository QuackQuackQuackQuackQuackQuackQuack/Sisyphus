use crate::expr::{ Expr, Lit };
use peg;
use unicode_ident::{is_xid_start, is_xid_continue};


struct StringTerminator(!);
impl StringTerminator {
    const IDENTIFIER : char = '`';
    const NORMAL     : char = '"';
}


peg::parser! { grammar sisyphys_parser() for str {

    rule args(count : usize) -> Vec<Expr>
        = { todo!() }

    rule lit() -> Lit
        = str:lit_string(StringTerminator::NORMAL) { Lit::String(str) }
    / b:lit_bool() { Lit::Bool(b) }
    / f:lit_float() { Lit::Float(f) }
    / int:lit_int() { Lit::Int(int) }

    rule lit_bool() -> bool
    = "true" { true } 
    / "false" { false }

    rule lit_float() -> f128
    = quiet!{
        f:$(lit_int() "." lit_int()) {? f.parse().or(Err("failed to parse a float"))
    }

    rule lit_int() -> i128
	= quiet!{
	    n:$(['0'..='9']+) {? n.parse().or(Err("failed to parse an integer")) }
	}
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
