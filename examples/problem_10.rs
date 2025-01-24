use project_euler_net::is_prime;

fn main() {
    let mut sum = 0;
    // The problem doesnt consider 0 and 1 as a prime
    for n in 2..2_000_000 {
        if is_prime(n) {
            sum += n;
        }
    }
    dbg!(sum);
}