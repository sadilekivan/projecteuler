pub fn is_prime(n: u64) -> bool {
    // We can start range at 2, this skips 0, 1, 2 because they are prime. End just before the number tested

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let mut n = 3;
    let mut i = 0;

    loop {
        if is_prime(n) {
            i += 1
        }

        if i == 10_000 {
            break;
        }

        n += 1;
    }

    dbg!(n);
}
