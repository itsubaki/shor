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
