/*
https://github.com/landrzejewski/rust-embedded

Installation/environment setup
- rustup tool from https://rustup.rs
- Visual Code + Rust extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format source a file
cargo new training_project             # create new project with cargo tool
cargo run                              # build and run an application in debug mode
cargo build                            # build an application in debug mode
cargo build --release                  # build an application in release mode
cargo clean                            # clean project
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
cargo clippy                           # check project with linter
*/
use crate::exercises::guess_game;

mod exercises;
mod language_basics;
mod memory_management;

fn main() {
    // language_basics::run();
    // fibonacci::run();
    // money::run();
    // money_with_enums::run();
    // memory_management::run();
    guess_game::run();
}
