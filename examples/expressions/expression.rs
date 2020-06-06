fn main() -> () {
  let mut x = {
    let a = 5i32;
    a + a + a * 2
  };

  println!("{:?}", x);

  let y = {
    "hello"
  };

  println!("{:?}", y);

  let z = loop {
    if x % 7 == 0 {
      break x;
    }

    x += 1;
  };

  println!("new value: {}", z);
}
