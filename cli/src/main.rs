// #![feature(exact_size_is_empty)]

use std::env;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let arg1 = env::args().nth(1).expect("no pattern given");
    println!("{:?} {:?}", env::args_os(), arg1);
    println!("{:?} {:?}", env::home_dir(), env::current_dir());

    test();
}

fn test() {
    let stdout = io::stdout();

    let mut handle = io::BufWriter::new(stdout.lock());
    writeln!(handle, "foo: {}", 42);
}
