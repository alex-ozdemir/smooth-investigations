extern crate clap;
extern crate num_bigint;
extern crate num_traits;
extern crate rand;

use clap::{App, Arg, SubCommand};
use num_bigint::{BigUint, RandomBits};
use num_traits::One;
use rand::Rng;

use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

trait Gen {
    fn next(&mut self) -> BigUint;
}

trait Permutation {
    fn permute(&self, int: &BigUint) -> BigUint;
}

struct Increment;

impl Permutation for Increment {
    fn permute(&self, int: &BigUint) -> BigUint {
        int + 1usize
    }
}

struct LeftRotate {
    total_bits: usize,
    rot_amount: usize,
    mask: BigUint,
}

impl LeftRotate {
    fn new(total_bits: usize, rot_amount: usize) -> Self {
        Self {
            total_bits,
            rot_amount,
            mask: (BigUint::one() << total_bits) - 1usize,
        }
    }
}

impl Permutation for LeftRotate {
    fn permute(&self, int: &BigUint) -> BigUint {
        ((int << self.rot_amount) & &self.mask) | (int >> (self.total_bits - self.rot_amount))
    }
}

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

impl<R: Rng> Gen for UniformGen<R> {
    fn next(&mut self) -> BigUint {
        self.rng.sample::<BigUint, _>(&self.dist) | (BigUint::one() << (self.bits - 1))
    }
}

struct PermutationGen<R, P> {
    bits: usize,
    seed: UniformGen<R>,
    permutation: P,
}

impl<P> PermutationGen<rand::rngs::ThreadRng, P> {
    fn new(bits: usize, seed_bits: usize, permutation: P) -> Self {
        Self {
            bits,
            seed: UniformGen::new(seed_bits),
            permutation,
        }
    }
}

impl<R: Rng, P: Permutation> Gen for PermutationGen<R, P> {
    fn next(&mut self) -> BigUint {
        let mut acc = self.seed.next();
        let mut last = acc.clone();
        let mut n_bits_left = self.bits - self.seed.bits;
        while n_bits_left > 0 {
            let n_bits_this_chunk = min(self.seed.bits, n_bits_left);
            n_bits_left -= n_bits_this_chunk;
            last = P::permute(&self.permutation, &last);
            let new = if n_bits_this_chunk < self.seed.bits {
                &last & ((BigUint::one() << n_bits_this_chunk) - 1usize)
            } else {
                last.clone()
            };
            acc = (acc << n_bits_this_chunk) | new;
        }
        acc
    }
}

fn main() {
    let matches = App::new("Non-Smooth Generator")
        .author("Alex Ozdemir")
        .about("Generates list of numbers that may or may not be smooth in various ways")
        .subcommand(
            SubCommand::with_name("uniform")
                .about("generates uniformly random numbers of a given length"),
        )
        .subcommand(
            SubCommand::with_name("permutation")
                .about(
                    "generates random numbers using a small uniform seed, and then a permutation",
                )
                .arg(
                    Arg::with_name("permutation")
                        .short("p")
                        .required(true)
                        .takes_value(true)
                        .help("the permutation to use: {increment, rotate}"),
                )
                .arg(
                    Arg::with_name("seed-bits")
                        .short("s")
                        .required(true)
                        .takes_value(true)
                        .help("how many bits to put in the seed"),
                ),
        )
        .arg(
            Arg::with_name("bits")
                .short("b")
                .required(true)
                .takes_value(true)
                .help("Number of bits in each number"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .required(true)
                .takes_value(true)
                .help("the number of numbers to generate"),
        )
        .get_matches();

    let number =
        usize::from_str(matches.value_of("number").unwrap_or("10")).expect("integral number");
    let mut numbers = HashSet::with_capacity(number);

    let bits = usize::from_str(matches.value_of("bits").unwrap_or("8")).expect("integral bits");
    let mut gen: Box<dyn Gen> = match matches.subcommand() {
        ("uniform", Some(_)) => Box::new(UniformGen::new(bits)),
        ("permutation", Some(m)) => {
            let seed_bits = usize::from_str(m.value_of("seed-bits").unwrap_or("8")).expect("integral seed-bits");
            match m.value_of("permutation").unwrap() {
                "increment" => {
                    Box::new(PermutationGen::new(bits, seed_bits, Increment))
                }
                "rotate" => {
                    Box::new(PermutationGen::new(bits, seed_bits, LeftRotate::new(seed_bits, 1)))
                }
                _ => panic!("Expected permutation"),
            }
        }
        _ => panic!("Unkown subcommand"),
    };

    while numbers.len() < number {
        let new = gen.next();
        numbers.insert(new);
    }
    for n in &numbers {
        println!("{:x}", n);
    }
}
