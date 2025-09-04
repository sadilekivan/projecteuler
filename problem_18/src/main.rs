use std::cmp;

enum NodeChild<'a> {
    Branch(Node<'a>),
    Leaf(u32),
}

struct Node<'a> {
    pub value: u32,
    pub a: Option<&'a Node<'a>>,
    pub b: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new_branch(value: u32, a: &'a Node, b: &'a Node) -> Self {
        Self {
            value,
            a: Some(a),
            b: Some(b),
        }
    }

    pub fn new_leaf(value: u32) -> Self {
        Self {
            value,
            a: None,
            b: None,
        }
    }

    pub fn traverse(&self) {
        dbg!(self.value);
        self.a.inspect(|a| a.traverse());
        self.b.inspect(|b| b.traverse());
    }
}

fn main() {
    let data = include_str!("input.data");
    let mut v: Vec<Vec<u32>> = Vec::new();

    for (y, line) in data.lines().enumerate() {
        v.push(Vec::new());
        for n in line.split_whitespace() {
            let n = n.parse().unwrap();
            v[y].push(n);
        }
    }
    dbg!(&v);

    let mut y = v.len() - 2;
    let mut x = 0;
    loop {
        // Sum from numbers below this number
        let a = v[y][x] + v[y + 1][x];
        let b = v[y][x] + v[y + 1][x + 1];
        v[y][x] = cmp::max(a, b);
        dbg!(v[y][x]);
        // Do the next number in row
        x += 1;
        // If done with row move up
        if x == v[y].len() {
            x = 0;
            // If done with all rows stop
            if y == 0 {
                break;
            }
            y -= 1;
        }
    }

    // let mut n = Node::new_leaf(v[top][0]);
    // let a = Node::new_leaf(v[bot][0]);
    // n.a = Some(&a);
    // let b = Node::new_leaf(v[bot][1]);
    // n.b = Some(&b);

    // n.traverse();
}
