pub fn is_prime(n: u64) -> bool {
    // We can start range at 2, this skips 0, 1, 2 because they are prime for sure and end just before the number tested
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}