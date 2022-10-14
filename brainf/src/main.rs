use std::time::Instant;

use parser::Parser;
use runner::Runner;

mod parser;
mod runner;

fn main() {
    let t0 = Instant::now();
    let mut parser = Parser::new();
    parser.parse("./program.bf".to_string());
    let t1 = Instant::now();
    let mut runner = Runner::new(parser.tokens);
    runner.run();
    let t2 = Instant::now();
    println!("parsing : {:?}", t1 - t0);
    println!("running : {:?}", t2 - t1);
    println!("total : {:?}", t2 - t0);
}
