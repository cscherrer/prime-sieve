use priority_queue::PriorityQueue;
use std::collections::VecDeque;
use std::time::Instant;

// A "filter" (nothing official here, just sounds good to me) is a sequence of
// multiples of some prime. In the Sieve of Eratosthenes, it's the sequence of
// "crossed out numbers" (p, 2p, 3p, ...) for any prime p.
//
// This could easily be made an Iterator, but we don't use that functionality so
// we leave it out.
#[derive(Hash, Copy, Clone, Eq, PartialEq)]
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
            base,
            state: base * base,
        }
    }

    fn step(&mut self) -> u64 {
        self.state += self.base;
        self.state
    }
}

// Naively we'd check every integer. But we can avoid checking even numbers by
// instead adding 2 at each step. To also avoid checking multiples of 3, we'd
// alternate adding 2 and 4.
//
// This is called a _wheel_ of size 2, with the pattern [2, 4]. We can
// generalize this to wheels of size n, with the pattern [2, 4, 2, 4, 6, 2...]
//
// At a point there are dimishing returns to increasing the size of the wheel,
// because prime number become less dense. But for small wheels, the speedup is
// significant.
//
// We use a wheel of size 48, which allows us to avoid checking multiples of 2,
// 3, 5, and 7.
const WHEEL_STATES: [u64; 48] = [
    2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2, 4, 6,
    2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
];

struct Wheel {
    index: usize,
    state: u64,
}

impl Wheel {
    fn new() -> Wheel {
        // Hack to make sure we start at 11 with index 0
        Wheel {
            index: 47,
            state: 1,
        }
    }

    fn next(&mut self) -> u64 {
        self.state += WHEEL_STATES[self.index];

        if self.index == 47 {
            self.index = 0;
        } else {
            self.index += 1;
        }

        self.state
    }
}

const SMALL_PRIMES: [u64; 4] = [2, 3, 5, 7];

// The BiggerPrimes struct is an iterator over prime numbers. It maintains a list of
// active filters, and a queue of filters (really a VecDeque) that are waiting
// to be activated.
//
// This queue is helpful because it's inefficient to constantly search a filter
// we know won't be useful until we're at the square of its base
struct BiggerPrimes {
    state: Wheel,
    active_filters: PriorityQueue<Filter, u64>,
    queued_filters: VecDeque<Filter>,
}

impl BiggerPrimes {
    pub fn new() -> BiggerPrimes {
        BiggerPrimes {
            state: Wheel::new(),
            active_filters: PriorityQueue::new(),
            queued_filters: VecDeque::new(),
        }
    }

    fn step(&mut self) -> Option<u64> {
        let n = self.state.next();

        // If any active filter matches, we're not prime
        while let Some((f, _)) = self.active_filters.peek() {
            match f.state {
                x if x < n => {
                    let mut f = self.active_filters.pop().unwrap().0;
                    f.step();
                    self.active_filters.push(f, f.state.wrapping_neg());
                }
                x if x == n => {
                    return None;
                }
                _ => {
                    break;
                }
            }
        }

        // Update queued filters. The first entry is always p^2, so at most one will need updating
        if n == self.queued_filters.front().map(|f| f.state).unwrap_or(0) {
            let f = self.queued_filters.pop_front().unwrap();
            self.active_filters.push(f, f.state.wrapping_neg());
            return None;
        }

        // If we reach this point, we know we're at a prime number. So queue a
        // new filter and return a Some
        self.queued_filters.push_back(Filter::new(n));
        Some(n)
    }
}

impl Iterator for BiggerPrimes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.step().is_none() {}
        Some(self.state.state)
    }
}

struct Primes {
    small_primes: std::slice::Iter<'static, u64>,
    bigger_primes: BiggerPrimes,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            small_primes: SMALL_PRIMES.iter(),
            bigger_primes: BiggerPrimes::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&prime) = self.small_primes.next() {
            Some(prime)
        } else {
            self.bigger_primes.next()
        }
    }
}

fn main() {
    let primes = &mut Primes::new();
    let now = Instant::now();
    // for p in primes.take(10) {
    //     println!("{}", p);
    // }
    let p = primes.nth(999999).unwrap();
    let t = now.elapsed().as_millis();
    println!("Millionth prime: {}", p);
    println!("Compute time:    {}ms", t);
}
