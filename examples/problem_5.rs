fn is_divisible_by_range(mut range: impl Iterator<Item = u32>, n: u32) -> bool {
    range.all(|x| n % x == 0)
}

fn main() {
    // We can start at 20 and increment by 20 since thats our highest divisor
    let highest_divisor = 20;
    
    let mut n = highest_divisor;
    loop {
        if is_divisible_by_range(1..=highest_divisor, n) {
            break;
        }
        n += highest_divisor;
    };
    dbg!(n);
}