use std::{env, fs::File, io::{BufReader, Read}};

pub mod operations;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: String = args[1].clone();

    println!("{}", operations::get_flesch_reading_test(path));
}
