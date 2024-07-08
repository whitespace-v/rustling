// module
use rand::Rng;

mod random_number;
//прелюдия
use crate::prelude::*;

use random_number::RandomNumber;
pub fn generate() -> RandomNumber {
    super::shared();
    let rand_num = rand::thread_rng().gen_range(LOW..=HIGH);
    RandomNumber::new(rand_num)
}

pub fn from_gen() {
    println!("fr_gn!")
}
