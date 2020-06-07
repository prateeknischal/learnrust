mod algorithms;
mod timer;

use std::time::Instant;

fn primes() -> () {
  let __t = timer::Timer::new();
  let _x = algorithms::primes(1000000);
}

fn main() -> () {
  println!("{}", algorithms::pow_mod(5, 6, 7));
  primes();
}
