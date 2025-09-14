pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32;

    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false
        }
    }
    true
}
