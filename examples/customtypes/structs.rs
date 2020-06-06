use std::fmt;

struct Complex {
  x: i32,
  y: i32
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} + i{}", &self.x, &self.y)
  }
}

fn main() -> () {
  let a:Complex = Complex{x:1, y:2};
  println!("out: {}", a);

  match a {
    Complex{x, y: 1} => println!("Match found: {}", x),
    Complex{x: 1, ..} => println!("x coordinate matched"),
    _ => println!("Nothing for you here")
  }
}
