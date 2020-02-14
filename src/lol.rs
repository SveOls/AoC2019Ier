#[derive(Debug)]
/// Fibonacci sequence. Contains current and next value, u64. Implements Iterator and From<usize>.
struct Fibonacci {
    curr: u64,
    next: u64,
    max: Option<u64>,
}

impl From<usize> for Fibonacci {
    ///input is the index of first element. 0 -> 0, 1, 1 | 1 -> 1, 1, 2 | 2 -> 1, 2, 3, etc
    fn from(inp: usize) -> Self {
        let mut a = Fibonacci {
            curr: 0,
            next: 1,
            max: None,
        };
        for _ in 0..inp {
            a.next();
        }
        a
    }
}
impl From<u64> for Fibonacci {
    fn from(inp: u64) -> Self {
        Fibonacci {
            curr: 0,
            next: 1,
            max: Some(inp),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let nw = self.next.checked_add(self.curr);
        self.curr = self.next;
        if let Some(a) = nw {
            self.next = a;
        } else if self.next == self.curr {
            return None;
        }
        if let Some(a) = self.max {
            if a < self.curr {
                return None;
            }
        }
        Some(self.curr)
    }
}