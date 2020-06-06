fn main() -> () {
  // variable bindings are immutable, to make it
  // mutable, use "mut".
  let mut _x: i32 = 1234;
  let tup: (i32, i32, f64) = (1, 2, 1.72);

  println!("{:?}", tup);

  _x = 1235;
  println!("{}", _x);
}
