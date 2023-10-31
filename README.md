# prime-sieve

A simple-ish prime number sieve. [The source code](https://github.com/cscherrer/prime-sieve/blob/master/src/main.rs) has lots of comments describing how this works.

It's fairly quick. `main` finds the millionth prime, like this:
```rust
fn main() {
    let now = Instant::now();
    let primes = &mut Primes::new();
    let p = primes.skip(999999).next().unwrap();
    let t = now.elapsed().as_millis();
    println!("Millionth prime: {}", p);
    println!("Compute time:    {}ms", t);
}
```

On my machine, the result is
```
Millionth prime: 15485863
Compute time:    538ms
```
