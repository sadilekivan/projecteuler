fn main() {
    let mut v: Vec<i32> = vec![];
    for n in 0..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            v.push(n);
        }
    }
    let sum: i32 = v.iter().sum();
    dbg!(sum);
}