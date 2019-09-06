extern crate clap;
extern crate rand;
extern crate num_bigint;
extern crate num_traits;

use clap::{Arg, App, SubCommand};
use num_bigint::{BigUint, RandomBits};
use num_traits::One;
use rand::Rng;

use std::collections::HashSet;
use std::str::FromStr;

struct UniformGen<R> {
    bits: usize,
    dist: RandomBits,
    rng: R,
}

impl UniformGen<rand::rngs::ThreadRng> {
    fn new(bits: usize) -> Self {
        Self {
            bits: bits,
            dist: RandomBits::new(bits - 1),
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> UniformGen<R> {
    fn next(&mut self) -> BigUint {
        self.rng.sample::<BigUint, _>(&self.dist) | (BigUint::one() << (self.bits - 1))
    }
}

fn main() {
    let matches = App::new("Non-Smooth Generator")
        .author("Alex Ozdemir")
        .about("Generates list of numbers that may or may not be smooth in various ways")
        .subcommand(SubCommand::with_name("uniform")
                    .about("generates uniformly random numbers of a given length")
                    .arg(Arg::with_name("bits")
                         .short("b")
                         .takes_value(true)
                         .help("Number of bits in each number")))
        .arg(Arg::with_name("number")
             .short("n")
             .takes_value(true)
             .help("the number of numbers to generate"))
        .get_matches();

    

    let number = usize::from_str(matches.value_of("number").unwrap_or("10")).expect("integral number");
    let mut numbers = HashSet::with_capacity(number);

    let bits = usize::from_str(matches.subcommand_matches("uniform").expect("Must have a subcommand!").value_of("bits").unwrap_or("8")).expect("integral bits");
    let mut gen = UniformGen::new(bits);

    while numbers.len() < number {
        let new = gen.next();
        numbers.insert(new);
    }
    for n in &numbers {
        println!("{:x}", n);
    }
}
