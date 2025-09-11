/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let mut sum = 0;
    let digits = code.chars().rev();
    let mut double = false;

    if code.trim().len() <= 1 {
        return false;
    }
    
    if code.chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false;
    }

    for i in digits {
        if let Some(n) = i.to_digit(10) {
            if double {
                let d = n * 2;
                if d > 9 {
                    sum += d - 9;
                    double = false;
                    continue;
                } else {
                    sum += d;
                    double = false;
                    continue;
                }
            }
            sum += n;
            double = !double;
        } else {
            continue;
        }

    }

    sum % 10 == 0
}
