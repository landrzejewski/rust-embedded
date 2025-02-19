use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ARG_PREFIX: &str = "-";
const NUMBERING_ARG: &str = "-n";
const NUMBERING_IGNORE_EMPTY_ARG: &str = "-nb";

enum Mode {
    Default,
    Numbering { empty: bool }
}

impl From<&String> for Mode {
    fn from(arg: &String) -> Self {
        match arg.as_ref() {
            NUMBERING_ARG => Mode::Numbering { empty: false },
            NUMBERING_IGNORE_EMPTY_ARG => Mode::Numbering { empty: true },
            _ => Mode::Default,
        }
    }
}

fn get_config() -> (Vec<String>, Vec<String>) {
    args().skip(1).partition(|arg| arg.starts_with(ARG_PREFIX))
}

fn show_help() {
    println!("Usage:");
    println!("cat [option] file1 file2 ...");
    println!("options:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers, ignore blank lines");
}

fn cat(mode: &Mode, paths: &Vec<String>) {
    let print_line = match mode {
        Mode::Numbering { empty: false } => print_with_numbering,
        Mode::Numbering { empty: true } => print_with_numbering_ignoring_empty,
        _ => print
    };
    for path in paths {
        let Ok(file) = File::open(path) else {
            eprintln!("File not found: {}", path);
            continue;
        };
        println!("File: {}", path);
        BufReader::new(file).lines()
            .enumerate()
            .for_each(|(idx, line)| print_line(idx + 1, &line.expect("Could not read line")));
    }
}

fn print(_line_number: usize, line: &String) {
    println!("{line}");
}

fn print_with_numbering(line_number: usize, line: &String) {
    println!("{:3}:\t{}", line_number, line);
}

fn print_with_numbering_ignoring_empty(line_number: usize, line: &String) {
    if line.is_empty() {
        println!();
    } else {
        print_with_numbering(line_number, line);
    }
}

pub fn run() {
    let (options, paths) = get_config();
    if paths.is_empty() {
        show_help();
        return;
    }
    let mode = options.get(0).map(Mode::from).unwrap_or(Mode::Default);
    cat(&mode, &paths);
}

