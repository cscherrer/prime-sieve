use std::collections::VecDeque;

// A "filter" (nothing official here, just sounds good to me) is a sequence of
// multiples of some prime. In the Sieve of Eratosthenes, it's the sequence of
// "crossed out numbers" (p, 2p, 3p, ...) for any prime p.
//
// This could easily be made an Iterator, but we don't use that functionality so
// we leave it out.
struct Filter {
    base: u64,
    state: u64,
}

impl Filter {
    // A new filter could naively start at p, but we can do better. We know that
    // p^2 is the first number in the filter that is not a multiple of any
    // smaller prime. So we start there.
    fn new(base: u64) -> Filter {
        Filter {
            base: base,
            state: base * base,
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

// The Primes struct is an iterator over prime numbers. It maintains a list of
// active filters, and a queue of filters (really a VecDeque) that are waiting
// to be activated. 
//
// This queue is helpful because it's inefficient to constantly search a filter
// we know won't be useful until we're at the square of its base
struct Primes {
    active_filters: Vec<Filter>,
    queued_filters: VecDeque<Filter>,
    state: u64,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            active_filters: Vec::new(),
            queued_filters: VecDeque::new(),
            state: 1,
        }
    }

    fn step(&mut self) -> Option<u64> {
        self.state += 1;
        
        // If any active filter matches, we're not prime
        if self.active_filters.iter_mut().any(|f| f.query(self.state)) {
            return None;
        }

        // If the next queued filter is ready, activate it
        if self.state == self.queued_filters.front().map(|f| f.state).unwrap_or(0) {
            self.active_filters
                .push(self.queued_filters.pop_front().unwrap());
            return None;
        }

        // If we reach this point, we know we're at a prime number. So queue a
        // new filter and return a Some
        self.queued_filters.push_back(Filter::new(self.state));
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
    for p in primes.skip(999999).take(1) {
        println!("{}", p);
    }
}
