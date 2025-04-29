use sisyphus;
use sisyphus::exec::Execute;


fn main() {
    match (sisyphus::parser::parse("print \"Hello, World!\"")) {
        Ok(script) => { for expr in script { expr.execute(); } },
        Err(err) => { err.print_formatted(); }
    }
}
