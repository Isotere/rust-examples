use rayon::prelude::*;
use std::time::Instant;

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|x| n % x != 0)
}

fn main() {
    let now = Instant::now();
    let numbers: Vec<u32> = (1..50_000).collect();

    let mut primes = numbers.par_iter().filter(|&x| is_prime(*x)).collect::<Vec<_>>();
    primes.par_sort_unstable();

    let elapsed = now.elapsed().as_secs_f32();

    println!("Found {} primes in {} seconds", primes.len(), elapsed);

    println!("Non parallel sorting:");
    let now = Instant::now();
    let mut primes = numbers.iter().filter(|&x| is_prime(*x)).collect::<Vec<_>>();
    primes.sort();
    let elapsed = now.elapsed().as_secs_f32();

    println!("Found {} primes in {} seconds", primes.len(), elapsed);
}
