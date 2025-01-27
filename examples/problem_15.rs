#[derive(Debug, Default, Clone, Copy)]
struct GridTraversal<const S: u8> {
    x: u8,
    y: u8,
}

impl<const S: u8> GridTraversal<S> {
    fn move_along(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Right => {
                if self.x == S {
                    return false;
                }
                self.x += 1;
            },
            Direction::Down => {
                if self.y == S {
                    return false;
                }
                self.y += 1
            },
        }
        return true;
    }

    fn at_finish(&self) -> bool {
        self.x == S && self.y == S
    }
}

enum Direction {
    Right,
    Down
}

fn traverse1<const S: u8>(traveler_v: &mut Vec<GridTraversal<S>>) {
    if traveler_v.iter().all(|t| t.at_finish()) {
        return;
    }
    
    for i in 0..traveler_v.len() {
        let child_a = &mut traveler_v[i];
        let mut child_b = child_a.to_owned();
        
        // Our traveler survives to the next vector if he can move in his direction
        if child_a.move_along(Direction::Right) {
            // this one is already in vec
            if child_b.move_along(Direction::Down) {
                traveler_v.push(child_b);
            }
        } else if child_a.move_along(Direction::Down) {
            // this one is already in vec
        }
    }

    traverse1(traveler_v)
}

fn traverse2<const S: u8>(x: u8, y: u8, count: &mut u64) {
    if x == S && y == S {
        *count += 1;
        return;
    }

    if x < S {
        traverse2::<S>(x + 1, y, count);
    }

    if y < S {
        traverse2::<S>(x, y + 1, count);
    }
}

struct LShapedIter{
    size: usize,
    index: usize
}

impl LShapedIter {
    fn new(size: usize) -> Self {
        Self { index: 0, size }
    }
}

impl Iterator for LShapedIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 2 * self.size - 2 {
            return None;
        }
        let result = if self.index > self.size - 1 {
            (0, self.index - self.size + 1)
        } else {
            (self.index, 0)
        };

        self.index += 1;
        Some(result)
    }
}

fn traverse3<const S: usize>(depth: usize, grid: &mut [[u64; S]; S]) -> u64 {
    if depth == S {
        return grid[S-1][S-1];
    }

    if depth == 0 {
        for (x, y) in LShapedIter::new(S) {
            grid[y][x] = 1;
        }
    } else {
        for (x, y) in LShapedIter::new(S - depth).map(|(x, y)| (x + depth, y + depth)) {
            let a = grid.get(y-1).and_then(|g| g.get(x));
            let b = grid.get(y).and_then(|g| g.get(x-1));
            if let (Some(a), Some(b)) = (a, b) {
                grid[y][x] = a + b; 
            }
        }
    }

    return traverse3::<S>(depth + 1, grid);
}

fn factorial(mut x: f64) -> f64 {
    for i in 2..x as u64 {
        x *= i as f64;
    }
    x
}

#[allow(non_snake_case)]
// trying out combinatorics
fn K(n: f64, k: f64) -> f64 {
    factorial(n) / (factorial(k)*factorial(n - k))
}

fn main() {
    // This approach is fun to simulate but takes too long for a grid of 20
    let mut traveler_v = vec![GridTraversal::<4>::default()];
    traverse1(&mut traveler_v);
    dbg!(traveler_v.len());
    
    // Also takes too long, too many combinations at 20
    let count: &mut u64 = &mut 0;
    traverse2::<4>(0, 0, count);
    dbg!(count);

    const S: usize = 20;
    let grid = &mut [[0u64; S+1]; S+1];
    let count = traverse3(0, grid); 
    dbg!(count);

    // There is a 40 long path of Rights or Downs, with always 20 Rights and 20 Downs
    // In other words how many ways can I put 20 Rights/Downs into a 40 places, the latter is the other (i.e. if I put 20 Rights the rest must be Downs, and if I put 20 Downs the rest is Rights)
    dbg!(K(40., 20.));
}