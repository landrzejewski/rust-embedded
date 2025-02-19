use std::env::args;

pub fn run() {
    args().skip(1).for_each(|arg| println!("{arg} "));
}