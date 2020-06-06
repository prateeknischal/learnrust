use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
// use std::num::IntErrorKind;

struct Complex {
  x: i32,
  y: i32,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} + i{}", &self.x, &self.y)
  }
}

impl FromStr for Complex {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let v:Vec<&str> = s.split(" + i").collect();
    // if v.len() != 2 {
    //   return Err(ParseIntError{kind: IntErrorKind::Empty});
    // }

    let x = v[0].parse::<i32>().unwrap();
    let y = v[1].parse::<i32>().unwrap();
    return Ok(Complex{x: x, y: y})
  }
}

fn main() -> () {
  let n: Complex = Complex{ x:1, y:2 };
  let marshal: &str = &n.to_string();
  println!("{}", marshal);

  println!(
    "{}", Complex::from_str(marshal).unwrap()
  )
}
