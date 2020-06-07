use std::time::Instant;

pub struct Timer {
  begin: Instant
}

impl Timer {
  pub fn new() -> Timer {
    Timer {
      begin: Instant::now()
    }
  }
}

impl Drop for Timer {
  fn drop(&mut self) {
    println!("Time taken by the scope: {:?}", self.begin.elapsed());
  }
}
