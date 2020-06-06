fn add_vec(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
  let mut c:(i32, i32, i32) = (0, 0, 0);

  c.0 = a.0 + b.0;
  c.1 = a.1 + b.1;
  c.2 = a.2 + b.2;

  return c
}

fn main() -> () {
  let a:(i32, i32, i32) = (1, 2, 3);
  let b:(i32, i32, i32) = (-3, -1, -2);

  println!("{:?}", add_vec(a, b));
}
