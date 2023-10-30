struct Filter {
    base: u64,
    state: u64
}

impl Filter {
    fn new(base: u64) -> Filter {
        Filter {
            base: base,
            state: base * base
        }
    }

    fn step(&mut self) -> u64 {
        self.state += self.base;
        self.state
    }

    fn query(&mut self, n: u64) -> bool {
        while self.state < n {
            self.step();
        }
        self.state == n
    }
}

struct Primes {
    filters: Vec<Filter>,
    state: u64
}

impl Primes {
    fn new() -> Primes {
        Primes {
            filters: Vec::new(),
            state: 1
        }
    }

    fn step(&mut self) -> Option<u64> {
        self.state += 1;
        for f in self.filters.iter_mut() {
            if f.query(self.state) {
                return None;
            }
        }
        self.filters.push(Filter::new(self.state));
        Some(self.state)
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let None = self.step() {}
        Some(self.state)
    }
}

fn main() {
    let primes = Primes::new();
    for p in primes.take(100) {
        println!("{}", p);
    }
}
