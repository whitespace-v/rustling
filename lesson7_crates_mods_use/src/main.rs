#![warn(clippy::all, clippy::pedantic)]

// binary crates -> main.rs & fn main
// library crates -> not fn main and doesn't work themselves, often include lib.rs

//all fn's are private by default

//rename standart name with alias:

// use generator::generate;
// to use "generate()" withot prefix "generator::"

// use generator::*; // all from generator

mod generator; // search file

//прелюдия для глабальной видимости констант
mod prelude {
    pub const LOW: u8 = 1;
    pub const HIGH: u8 = 200;
    pub use crate::generator::{from_gen, generate};
    pub use std::env as enviroment;
}
use prelude::*;

fn main() {
    let random = generate();
    let args: Vec<String> = enviroment::args().collect();
    let arg = &args[0];
    println!("{}, {arg}", random.value);
}

pub fn shared() {
    println!("shared!")
}
