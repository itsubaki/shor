use num::integer;
use rand::prelude::*;

pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    for i in 3..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn coprime(n: i32) -> i32 {
    let mut rng = rand::thread_rng();

    loop {
        let a: i32 = rng.gen_range(2..n - 1);
        if integer::gcd(n, a) == 1 {
            return a;
        }
    }
}
