mod puzzle01;
mod puzzle02;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle_name = args[1].as_str();

    match puzzle_name {
        "puzzle01" => puzzle01::run(),
        "puzzle02" => puzzle02::run(),
        _ => panic!("{puzzle_name} does not exist.")
    }
}
