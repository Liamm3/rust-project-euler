pub fn run() {
    let primes = Primes::new();
    const LIMIT: i64 = 600851475143;
    for prime in primes {
        println!("First Prime: {}", prime);
        if prime > 10 {
            break;
        }
    }
}

struct Primes {
    current: u64,
    next: u64,
}

impl Primes {
    fn new() -> Self {
        Self {
            current: 2,
            next: 3, 
        }
    }

    pub fn is_prime(n: u64) -> bool {
      if n < 4 {
        n > 1
      } else if n % 2 == 0 || n % 3 == 0 {
        false
      } else {
        let max_p = (n as f64).sqrt().ceil() as u64;
        match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
          Some(_) => false,
          None => true
        }
      }
    }

}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.current;
        self.current = self.next; 
        loop {
            self.next += match self.next % 6 {
                1 => 4,
                _ => 2,
            };
        }
    }
}
