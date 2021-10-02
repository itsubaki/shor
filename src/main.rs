use std::env;

mod number;
mod quantum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap(); // positive integer
    let t: u32 = args[3].parse().unwrap(); // precision bits
    let mut a: u32 = args[2].parse().unwrap(); // coprime number of n

    if n < 2 {
        println!("N={}. N must be greater than 1.", n);
        return;
    }

    if n % 2 == 0 {
        println!("N={} is even. p={}, q={}.", n, 2, n / 2);
        return;
    }

    if number::is_prime(n) {
        println!("N={} is prime.", n);
        return;
    }

    let (b, e, ok) = number::base_exp(n);
    if ok {
        println!("N={}. N is exponentiation. {}^{}.", n, b, e);
        return;
    }

    if a < 1 {
        a = number::coprime(n);
    }

    if a < 2 || a > n - 1 {
        println!("N={}, a={}. a must be 1 < a < N.", n, a);
        return;
    }

    if number::gcd(n, a) != 1 {
        println!("N={}, a={}. a is not coprime.", n, a,);
        return;
    }

    println!("N: {}, a: {}", n, a);

    let mut qsim = quantum::new();
    let r0 = qsim.zero_with(t);
    let r1 = qsim.zero_log2(n);

    qsim.x(&[r1[r1.len() - 1]]);
    qsim.h(&r0);
    qsim.cmodexp2(a, n, &r0, &r1);
    qsim.iqft(&r0);

    println!("{:?} {:?} {:?}", qsim, r0, r1);
}
