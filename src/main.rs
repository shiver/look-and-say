#![feature(test)]
extern crate test;

use std::time::{Instant, Duration};
use std::env;

fn dual_vecs(digits: &[u8], max: u8) {
    let mut vec1 = Vec::with_capacity(1_000_000_000);
    let mut vec2 = Vec::with_capacity(1_000_000_000);
    for d in digits.iter() {
        vec1.push(*d);
    }


    let mut count;
    let mut last_idx;
    let mut iterations = 0;

    loop {
        if iterations > max { break; }
        iterations += 1;

        vec2.clear();

        count = 0;
        last_idx = vec1.len() - 1;

        for (idx, &digit) in vec1.iter().enumerate() {
            if idx > 0 && digit != vec1[idx-1] {
                vec2.push(count);
                vec2.push(vec1[idx-1]);
                count = 1;
            } else {
                count += 1;
            }

            if idx >= last_idx {
                vec2.push(count);
                vec2.push(digit);
            }
        }

        vec1 = vec2.clone();
    }
}

fn sliced(digits: &[u8]) -> Vec<u8> {
    let mut count = 1;
    let mut current = digits[0];
    let mut new_digits = Vec::with_capacity(digits.len() ^ 4);

    for next_digit in &digits[1..] {
        if *next_digit != current {
            new_digits.push(count);
            new_digits.push(current);
            current = *next_digit;
            count = 1;
        } else {
            count += 1;
        }
    }

    new_digits.push(count);
    new_digits.push(current);

    new_digits
}

fn time<F, T>(f: F) -> Duration
  where F: FnOnce() -> T {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn main() {
    let iterations: u8 = env::args().nth(1).expect("iterations?").parse().expect("int?");
    println!("{} iterations", iterations);

    let seed = vec![1];

    let mut duration = time(||dual_vecs(&seed, iterations));
    println!("dual_vecs={}.{}", duration.as_secs(), duration.subsec_nanos());

    let mut new_digits = seed;
    duration = time(||
        for _ in 0..iterations+1 {
            new_digits = sliced(&new_digits)
        }
    );
    println!("slices={}.{}", duration.as_secs(), duration.subsec_nanos());
}
