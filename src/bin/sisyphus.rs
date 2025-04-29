use sisyphus;


fn main() {
    match (sisyphus::parser::parse("print ")) {
        Ok(script) => { println!("{:?}", script); },
        Err(err) => { err.print_formatted(); }
    }
}
