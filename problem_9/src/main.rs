fn is_pythagorian_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2) && a < b  && b < c
}

fn main() {
    /*let mut a = 1;
    let mut b = 2;
    let mut c = 3;
    loop {
        if c < 1000 - a - b {
            c += 1;
        } else {
            
        }
        if b < 1000 - a - c {
            b += 1;
        } else if a < 1000 - b - c {
            a += 1;
        } else {
            break;
        }
        dbg!((a, b, c));
    }*/

    // Ay lets just fry eggs on the cooler
    let range = 0..1000;
    let mut result = Default::default();
    for a in range.to_owned() {
        for b in range.to_owned() {
            for c in range.to_owned() {
                if is_pythagorian_triplet(a, b, c) && a + b + c == 1000 {
                    result = Some(a * b * c);
                }
            }
        }
    }

    dbg!(result);
}