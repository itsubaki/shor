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

pub fn modexp(a: u32, r: u32, n: u32) -> u32 {
    if a == 0 {
        return 0;
    }

    if r == 0 {
        return 1;
    }

    let mut p = a;
    for _ in 1..r {
        p = (p * a) % n
    }

    p
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

pub fn continued_fraction(f: f64) -> Vec<u32> {
    let mut list: Vec<u32> = vec![];
    let mut r: f64 = f;

    loop {
        let t: f64 = r.trunc();
        list.push(t as u32);

        let diff: f64 = r - t;
        if diff < 1e-3 {
            break;
        }

        r = 1.0 / diff;
    }

    list
}

pub fn convergent(cf: &[u32]) -> (u32, u32) {
    let len: usize = cf.len();
    if len == 1 {
        return (cf[0], 1);
    }

    let mut s: u32 = 1;
    let mut r: u32 = cf[len - 1];
    for i in 2..len {
        let tmp = s;
        s = r;
        r = cf[len - i] * r + tmp;
    }
    s += cf[0] * r;

    (s, r)
}

pub fn to_float(bin: &[char]) -> f64 {
    let mut f: f64 = 0.0;

    for (i, b) in bin.iter().enumerate() {
        if *b == '0' {
            continue;
        }

        f += 0.5_f64.powf((i + 1) as f64);
    }

    f
}

pub fn find_order(a: u32, n: u32, bin: &[char]) -> (u32, u32, bool) {
    if bin.is_empty() {
        return (0, 1, false);
    }

    let fv: f64 = to_float(bin);
    let cf: Vec<u32> = continued_fraction(fv);

    for i in 0..cf.len() {
        let (s, r) = convergent(&cf[0..(i + 1)]);

        if modexp(a, r, n) == 1 {
            return (s, r, true);
        }
    }

    let (s, r) = convergent(&cf);
    (s, r, false)
}

pub fn is_trivial(n: u32, factor: &[u32]) -> bool {
    for (_, p) in factor.iter().enumerate() {
        if 1 < *p && *p < n && n % p == 0 {
            return false;
        }
    }

    true
}

pub fn is_odd(n: u32) -> bool {
    if n % 2 == 0 {
        return false;
    }

    true
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

#[test]
fn test_continued_fraction() {
    assert_eq!(continued_fraction(0.0), [0]);
    assert_eq!(continued_fraction(1.0 / 16.0), [0, 16]);
    assert_eq!(continued_fraction(4.0 / 16.0), [0, 4]);
    assert_eq!(continued_fraction(7.0 / 16.0), [0, 2, 3, 1, 1]);
    assert_eq!(continued_fraction(13.0 / 16.0), [0, 1, 4, 3]);
    assert_eq!(continued_fraction(0.42857), [0, 2, 2, 1]);
    assert_eq!(continued_fraction(0.166656494140625), [0, 6]);
}

#[test]
fn test_convergent() {
    assert_eq!(convergent(&continued_fraction(1.0 / 16.0)), (1, 16));
    assert_eq!(convergent(&continued_fraction(4.0 / 16.0)), (1, 4));
    assert_eq!(convergent(&continued_fraction(7.0 / 16.0)), (7, 16));
    assert_eq!(convergent(&continued_fraction(13.0 / 16.0)), (13, 16));
    assert_eq!(convergent(&continued_fraction(0.42857)), (3, 7));
    assert_eq!(convergent(&continued_fraction(0.166656494140625)), (1, 6));
}

#[test]
fn test_find_order() {
    assert_eq!(find_order(7, 15, &['0', '1', '0']), (1, 4, true));
    assert_eq!(find_order(7, 15, &['1', '0', '0']), (1, 2, false));
    assert_eq!(find_order(7, 15, &['1', '1', '0']), (3, 4, true));
}
