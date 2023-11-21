use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Clone> Trinity<T> {
    fn rotate(&mut self) {
        // a b c - Initial
        mem::swap(&mut self.a, &mut self.b);
        // b a c
        mem::swap(&mut self.b, &mut self.c);
        // b c a - Rotated
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq> Solver<T> {
    fn resolve(&mut self) {
        // Solve inplace
        self.unsolved.retain_mut(|t| {
            for _ in 0..3 {
                t.rotate();
                if *t == self.expected {
                    return false;
                }
            }
            true
        })
    }
}
