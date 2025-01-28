fn is_palindrome(data: &str) -> bool {
    let ch_v: Vec<char> = data.chars().collect();
    let (ch_v1, ch_v2) = ch_v.split_at(ch_v.len() / 2);
    //dbg!(&ch_v, &ch_v1, &ch_v2);

    ch_v1.iter().zip(ch_v2.iter().rev()).all(|(a, b)| a == b)
}

fn main() {
    let mut max_palindrome: u32 = 0;

    let range = 100..=999; 

    for a in range.to_owned() {
        for b in range.to_owned() {
            let n: u32 = a * b;
            if is_palindrome(&n.to_string()) {
                if n > max_palindrome {
                    max_palindrome = n;
                }
            }
        }
    }

    dbg!(max_palindrome);
}