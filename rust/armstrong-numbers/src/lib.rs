pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let power = num.to_string().len().try_into().unwrap();

    for c in num.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        sum += digit.pow(power);
    }

    sum == num
}
