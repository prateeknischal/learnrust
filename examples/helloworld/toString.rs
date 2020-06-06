use std::fmt;

#[derive(Debug)]
struct Person<'a>(&'a str, i32);

impl fmt::Display for Person<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} of Age {}", self.0, self.1)
  }
}

fn main() -> () {
  let v = Person{0:"Prateek", 1:26};
  println!("Serialized using display: {}", v);
  println!("Serialized using debug  : {:?}", v);
}
