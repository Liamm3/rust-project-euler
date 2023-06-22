struct Fibs {
    a: u64,
    b: u64,
}

impl Fibs {
    fn new() -> Self {
        Fibs {
            a: 0,
            b: 1,
        }
    }
}

impl Iterator for Fibs {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.a + self.b;
        self.a = self.b;
        self.b = sum;
        Some(sum)
    }
}

pub fn run() {
    let fibs = Fibs::new();
    const LIMIT: u64 = 4000000;

    let sum: u64 = fibs.map_while(|fib| {
        if fib > LIMIT {
            return None;
        }

        Some(fib)
    }).filter(|n| n % 2 == 0).sum();

    println!("{}", sum);
}
