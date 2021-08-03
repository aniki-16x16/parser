pub mod json;

use std::{env, fs, process, time::Instant};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        process::exit(1);
    }
    let path = &args[1];
    let content = String::from_utf8(fs::read(path).unwrap()).unwrap();
    let now = Instant::now();
    let (_, result) = json::parse(&content).unwrap();
    let duration = now.elapsed().as_micros();
    println!("{:#?}\n{}μs", result, duration);
}
