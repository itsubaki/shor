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

    true
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
    let s: usize = format!("{:b}", n).chars().count();

    for i in (2..s).rev() {
        let a: f64 = (n as f64).powf(1.0 / (i as f64));
        if (a as u32).pow(i as u32) == n {
            return (a as u32, i as u32, true);
        }
    }

    (0, 0, false)
}

pub fn gcd(a: u32, b: u32) -> u32 {
    integer::gcd(a, b)
}

pub fn modexp2(a: u32, j: u32, n: u32) -> u32 {
    if a == 0 {
        return 0;
    }

    if j == 0 {
        return a % n;
    }

    let mut p = a;
    for _ in 0..j {
        p = (p * p) % n
    }

    p
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

#[test]
fn test_modexp2() {
    assert_eq!(modexp2(7, 0, 15), 7);
    assert_eq!(modexp2(7, 1, 15), 4);
    assert_eq!(modexp2(7, 2, 15), 1);
    assert_eq!(modexp2(7, 3, 15), 1);
    assert_eq!(modexp2(7, 4, 15), 1);
    assert_eq!(modexp2(7, 5, 15), 1);
    assert_eq!(modexp2(7, 6, 15), 1);
    assert_eq!(modexp2(7, 7, 15), 1);
    assert_eq!(modexp2(7, 8, 15), 1);
    assert_eq!(modexp2(7, 9, 15), 1);
    assert_eq!(modexp2(7, 10, 15), 1);
    assert_eq!(modexp2(7, 11, 15), 1);
    assert_eq!(modexp2(7, 12, 15), 1);
    assert_eq!(modexp2(7, 13, 15), 1);
    assert_eq!(modexp2(7, 14, 15), 1);
    assert_eq!(modexp2(0, 15, 15), 0);
}
