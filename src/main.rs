mod number;
mod quantum;

// cargo run 15
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args[1].parse().unwrap(); // positive integer
    let mut t: u32 = 3; // precision bits

    if args.len() > 2 {
        t = args[2].parse().unwrap();
    }

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

    if let Some((base, exp)) = number::base_exp(n) {
        println!("N={}. N is exponentiation. {}^{}.", n, base, exp);
        return;
    };

    let mut used = vec![];
    loop {
        let a = number::coprime(n);
        if used.contains(&a) {
            continue;
        }
        used.push(a);

        println!("N: {} (a: {}, t: {})", n, a, t);

        let mut qsim = quantum::Q::new();
        let r0 = qsim.zero_with(t);
        let r1 = qsim.zero_log2(n);

        qsim.x(&[r1[r1.len() - 1]]);
        qsim.h(&r0);
        qsim.cmodexp2(a, n, &r0, &r1);
        qsim.iqft(&r0);

        print(&qsim);
        println!();

        let mut rate = 0.0;
        for state in qsim.state().iter() {
            let m0 = state.to_binary_chars(&r0);

            let (_s, _r, ok) = number::find_order(a, n, &m0);
            if !ok || number::is_odd(_r) {
                println!("{}; s/r={:>2}/{:>2};", state, _s, _r);
                continue;
            }

            let p0 = number::gcd(a.pow(_r / 2) - 1, n);
            let p1 = number::gcd(a.pow(_r / 2) + 1, n);

            if number::is_trivial(n, &[p0, p1]) {
                println!("{}; s/r={:>2}/{:>2}; p={}, q={}", state, _s, _r, p0, p1);
                continue;
            }

            println!("{}; s/r={:>2}/{:>2}; p={}, q={}", state, _s, _r, p0, p1);
            rate += state.prob;
        }

        if rate > 0.0 {
            println!("success rate: {}", rate);
            break;
        }

        println!();
    }
}

fn print(qsim: &quantum::Q) {
    let max = max(qsim.state());
    for state in qsim.state().iter() {
        let size = (state.prob / max * 32.0) as usize;
        println!("{}; {}", state, (0..size).map(|_| "*").collect::<String>());
    }
}

fn max(states: Vec<quantum::State>) -> f64 {
    let mut max = 0.0;
    for state in states.iter() {
        if state.prob > max {
            max = state.prob;
        }
    }

    max
}
