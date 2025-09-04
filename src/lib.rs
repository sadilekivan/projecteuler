use std::collections::HashSet;

#[derive(Debug)]
pub struct TriangleNumberIter {
    n: u64,
    incerment: u64,
}

impl Default for TriangleNumberIter {
    fn default() -> Self {
        Self { n: 1, incerment: 2 }
    }
}

impl Iterator for TriangleNumberIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.n;

        self.n += self.incerment;
        self.incerment += 1;

        Some(result)
    }
}

#[test]
fn triangle_number_iterator() {
    let mut tni = TriangleNumberIter::default();

    for i in 1..50_000 {
        let triangle_n = (1..=i).sum::<usize>() as u64;
        assert_eq!(tni.next(), Some(triangle_n));
    }
}

// To optimize this method I used https://www.dcode.fr/divisors-list-number
// We use the square root of `n` method
pub fn factors_of_n(n: u64) -> HashSet<u64> {
    if n == 0 {
        return HashSet::from([]);
    }

    let mut factors_set = HashSet::from([1, n]);

    let top = (n as f64).sqrt() as u64;
    for d in 2..=top {
        if n % d == 0 {
            // Divisible by d, let's push it
            factors_set.insert(d);
            // But if n is divisible by d, the result is also a divisor
            let x = n / d;

            // By using a HashSet we account for result being the same as d or some other divisor
            factors_set.insert(x);
        }
    }

    factors_set
}
