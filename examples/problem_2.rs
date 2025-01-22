fn fib(index: usize) -> u64 {
    let mut sum = 0;

    let mut a = 1;
    let mut b = 0;
    
    for _ in 0..index {
        sum = a + b;
        a = b;
        b = sum;
    }
    sum
}

fn main() {
    let mut i = 0;
    let mut n;
    let mut sum = 0;
    loop {
        n = fib(i);
        if n > 4_000_000 {
            break;
        }
        dbg!(n);
        if n % 2 == 0 {
            sum += n;
        }
        i += 1;
    }
    dbg!(sum);
}