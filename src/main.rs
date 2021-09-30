use std::env;
mod number;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1: &String = &args[1];
    let n: i32 = arg1.parse().unwrap();

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

    // TODO: number::base_exp(n)

    let a: i32 = number::coprime(n);

    println!("N: {}, a: {}", n, a)
}
