use std::collections::HashMap;

fn primes(max: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for i in 2..=max {
        if primes.iter().any(|p| i % p == 0) {
            continue;
        }

        primes.push(i);
    }

    primes
}

// prime_num -> count
fn prime_factors(val: u64, primes: &Vec<u64>) -> HashMap<u64, u32> {
    let mut val = val;
    let mut prime_factors = HashMap::new();
    for prime in primes {
        if val == 1 {
            break;
        }

        let mut count = 0;
        while val % prime == 0 {
            val /= prime;
            count += 1;
        }

        if count != 0 {
            prime_factors.insert(*prime, count);
        }
    }

    prime_factors
}

// prime_number -> count
fn union_prime_factors(factors_list: &Vec<HashMap<u64, u32>>) -> HashMap<u64, u32> {
    let mut union = HashMap::new();
    for factors in factors_list {
        for (f, c) in factors {
            let entry = union.entry(*f).or_insert(*c);
            *entry = u32::max(*entry, *c);
        }
    }

    union
}

pub fn lcm(nums: &Vec<u64>) -> u64 {
    let primes = primes(*nums.into_iter().max().unwrap());
    let prime_factors: Vec<_> = nums.iter().map(|n| prime_factors(*n, &primes)).collect();

    let union = union_prime_factors(&prime_factors);
    let lcm = union.into_iter().map(|(p, n)| p * n as u64).product();

    lcm
}
