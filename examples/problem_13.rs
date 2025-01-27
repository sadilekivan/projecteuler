/// This one would be too simple with a BigInt impelentation from a crate, so I played around here with mine
use std::{cmp, iter, ops::{Add, AddAssign}};

#[derive(Debug)]
struct BigInt(String);

impl Default for BigInt {
    fn default() -> Self {
        Self(String::from("0"))
    }
}

fn add_big_number(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry_one: bool = false;

    // Working with iterators in reverse, just like adding on paper, padding with zeroes allows us to just zip them.
    let count = cmp::max(a.len(), b.len());
    let iter_a = a.chars().rev().chain(iter::repeat('0').take(count - a.len()));
    let iter_b = b.chars().rev().chain(iter::repeat('0').take(count - b.len()));

    for (a, b) in iter_a.zip(iter_b) {
        // Unsafe to just unwrap a possible digit, but for the sake of the problem I assume the input is okay
        // Also we could check input at making of `BigInt` so we are sure it contains digits only
        let mut total = a.to_digit(10).unwrap() + b.to_digit(10).unwrap();
        if carry_one {
            total += 1;
            carry_one = false;
        }

        if total >= 10 {
            total -= 10;
            carry_one = true;
        }
        result.push(char::from_digit(total, 10).unwrap());
    }

    if carry_one {
        result.push('1');
    }

    result.chars().rev().collect()
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        BigInt(add_big_number(&self.0, &rhs.0))
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = BigInt(add_big_number(&self.0, &rhs.0))
    }
}


fn main() {
    let mut total = BigInt::default();
    
    for n in include_str!("problem_13.data").lines().map(|line| BigInt(line.to_owned())) {
        total += n;
    }

    // Entire number
    dbg!(&total);

    // First ten digits
    dbg!(&total.0[0..10]);
}