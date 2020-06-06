use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = &self.0;
    for (i, value) in v.iter().enumerate() {
      write!(f, "{}:{} ", i, value)?;
    }
    write!(f, "")
  }
}

fn main() -> () {
  let v = List(vec![1, 2, 3]);
  println!("{}", v);
}
