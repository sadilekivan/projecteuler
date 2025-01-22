use project_euler_net::is_prime;

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