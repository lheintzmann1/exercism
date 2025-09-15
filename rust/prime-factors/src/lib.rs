pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();

    loop {
        if n % 2 != 0 {
            break;
        }

        n /= 2;
        factors.push(2);
    }
    
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            factors.push(i);
        }
        i += 2;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
