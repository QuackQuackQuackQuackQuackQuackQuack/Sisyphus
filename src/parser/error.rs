use peg::error::{ ParseError, ExpectedSet };
use peg::str::LineCol;


pub struct ParserError<'l> {
    line     : &'l str,
    column   : usize,
    expected : ExpectedSet
}

impl<'l> ParserError<'l> {
    pub(super) fn from_peg(line : &'l str, error : ParseError<LineCol>) -> Self {
        Self { line, column : error.location.column - 1, expected : error.expected }
    }
}

impl ParserError<'_> {
    pub fn print_formatted(&self) {
        println!("\x1b[0m\x1b[97m\x1b[101m\x1b[1m Failed to parse line \x1b[0m");
        println!("\x1b[96m{}\x1b[0m", self.line);
        println!("{}\x1b[93m\x1b[1m^\x1b[0m", " ".repeat(self.column));
        match (self.expected.tokens().count()) {
            0   => unreachable!(),
            1   => println!("\x1b[91m\x1b[1mExpected {}\x1b[0m", self.expected.tokens().next().unwrap()),
            2.. => println!("\x1b[91m\x1b[1mExpected one of {}\x1b[0m", self.expected.tokens()
                .intersperse(", ")
                .collect::<String>()
            )
        }
    }
}
