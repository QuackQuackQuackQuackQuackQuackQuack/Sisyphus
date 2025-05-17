use sisyphus;
use sisyphus::expr::{ Expr, Lit };
use sisyphus::exec::Executor;


fn main() {
    let source_file = "samples/big_list.push";

    let mut executor = Executor::new();
    executor.push_exprs([
        Expr::Push(Box::new((
            Expr::Lit(Lit::ExprQueue),
            Expr::FSRead(Box::new(
                Expr::Lit(Lit::String(source_file.to_string()))
            ))
        )))
    ]);

    while (executor.tick()) { }
}
