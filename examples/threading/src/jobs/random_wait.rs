use std::fs::File;
use std::io::prelude::Read;
use std::thread;
use std::time::{Duration, Instant};

fn get_rand() -> u64 {
  let mut f = File::open("/dev/urandom").unwrap();
  let mut buf: [u8; 1] = [0u8; 1];
  f.read(&mut buf).unwrap();

  return buf[0] as u64;
}

pub fn random_wait() {
  let mut v = Vec::new();
  let begin = Instant::now();
  for _ in 1..10 {
    v.push(thread::spawn(|| {
      thread::sleep(Duration::from_millis(1000u64 + get_rand()));
    }));
  }

  for t in v {
    t.join().unwrap();
  }

  println!("Time taken for random_wait: {:?}", begin.elapsed());
}
