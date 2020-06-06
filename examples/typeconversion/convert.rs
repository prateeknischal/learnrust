use std::convert::From;
use std::convert::TryFrom;
// use std::convert::TryInto;

#[derive(Debug)]
struct CorpNumber {
  number: i32
}

impl From<i64> for CorpNumber {
  fn from(item: i64) -> CorpNumber {
    return CorpNumber{ number: item as i32 }
  }
}

impl TryFrom<i32> for CorpNumber {
  type Error = ();

  fn try_from(item: i32) -> Result<Self, Self::Error> {
    if item & 1 == 1 {
      return Err(())
    } else {
      return Ok(CorpNumber{ number: item });
    }
  }
}

fn main() -> () {
  let s = "Hello world";
  let t:String = String::from(s);

  println!("The String {}", t);

  let v = 5i64;
  println!("The new string: {:?}", CorpNumber::from(v));

  let u: CorpNumber = v.into();
  println!("The converted string {:?}",u);

  let try_conv: Result<CorpNumber, ()> = CorpNumber::try_from(8i32);
  println!("TryFrom conversion: {:?}", try_conv);
}
