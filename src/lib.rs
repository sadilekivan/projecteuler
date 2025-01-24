pub fn is_prime(n: u64) -> bool {
    // We can start range at 2, this skips 0, 1, 2 because they are prime for sure and end just before the number tested
    
    let mut i = 2;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}