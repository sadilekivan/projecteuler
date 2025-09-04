// Perform Collatz operation on `n`
fn collatz_op(n: u64) -> u64 {
    if n % 2 == 0 {
        // Even operation
        n / 2
    } else {
        // Odd operation
        3 * n + 1
    }
}

fn main() {
    let mut result = 0;
    let mut max_count = 0;

    for start_n in 1..1_000_000 {

        let mut n = start_n;
        // Infinite for loop with iter count
        for count in 0.. {
            n = collatz_op(n);
            if n == 1 {
                // Capture sequence length and start_n
                if count > max_count {
                    result = start_n;
                    max_count = count;
                }
                // Sequence ended, break out
                break;
            }
        }    
    }

    dbg!(result, max_count);
}