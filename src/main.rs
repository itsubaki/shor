use std::env;

mod number;
mod quantum;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].parse().unwrap();
    let mut a: i32 = args[2].parse().unwrap();

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

    if a < 0 {
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

    // TODO shor
    let mut qsim = quantum::Q::new();
    let q0 = qsim.zero();

    qsim.x(&[q0]);
    qsim.h(&[q0]);

    println!("{:?} {:?}", qsim, q0);
}
