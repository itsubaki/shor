use num::integer;
use rand::prelude::*;

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    for i in 3..(integer::sqrt(n) + 1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn coprime(n: u32) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let a: u32 = rng.gen_range(2..n - 1);
        if gcd(n, a) == 1 {
            return a;
        }
    }
}

pub fn base_exp(n: u32) -> (u32, u32, bool) {
    let s = format!("{:b}", n).chars().count();
    for i in (2..s).rev() {
        let a: f64 = (n as f64).powf(1.0 / (i as f64));

        if (a as u32).pow(i as u32) == n {
            return (a as u32, i as u32, true);
        }
    }

    return (0, 0, false);
}

pub fn gcd(a: u32, b: u32) -> u32 {
    return integer::gcd(a, b);
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
}

#[test]
fn test_coprime() {
    assert!(gcd(15, coprime(15)) == 1);
    assert!(gcd(21, coprime(21)) == 1);
    assert!(gcd(35, coprime(35)) == 1);
    assert!(gcd(51, coprime(51)) == 1);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(15, 2), 1);
    assert_eq!(gcd(15, 3), 3);
    assert_eq!(gcd(15, 4), 1);
    assert_eq!(gcd(15, 5), 5);
    assert_eq!(gcd(15, 6), 3);
    assert_eq!(gcd(15, 7), 1);
    assert_eq!(gcd(15, 8), 1);
    assert_eq!(gcd(15, 9), 3);
    assert_eq!(gcd(15, 10), 5);
    assert_eq!(gcd(15, 11), 1);
    assert_eq!(gcd(15, 12), 3);
    assert_eq!(gcd(15, 13), 1);
    assert_eq!(gcd(15, 14), 1);
}

#[test]
fn test_base_exp() {
    assert_eq!(base_exp(25), (5, 2, true));
    assert_eq!(base_exp(27), (3, 3, true));
    assert_eq!(base_exp(36), (6, 2, true));
    assert_eq!(base_exp(49), (7, 2, true));
}
