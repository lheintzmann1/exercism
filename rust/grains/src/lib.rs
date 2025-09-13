pub fn square(s: u32) -> u64 {
    u64::pow(2, s-1)
}

pub fn total() -> u64 {
    let mut sum = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum
}
