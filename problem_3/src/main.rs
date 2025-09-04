fn main() {
    let mut i = 2;
    let mut n: u64 = 600_851_475_143;
    //let mut n: u64 = 13195;
    loop {
        if n % i == 0 {
            n /= i; // Thanks to this we skip many iterations and also dont need to check with is_prime
            dbg!(i);
        }
        if i > n {
            break;
        }
        i += 1;
    }
}
