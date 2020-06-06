use std::string;

fn ptr_magic(a: string::String, b: &string::String, c: &mut string::String) ->() {
  println!("a dies: {}", a);
  println!("b will survive: {}", b);
  c.push_str("hello");
  println!("c is mutable: {}", c);

}

fn main() -> () {
  let x = 1;
  println!("x: {}", x);

  let ref mut x = 1;
  *x += 1;
  println!("x: {}", x);

  let a = String::from("scoped out");
  let ref b = String::from("read only");
  let ref mut c = String::from("mutable");

  ptr_magic(a, b, c);

  // Will fail because a has been std::move()d
  // println!("a: {}", a);
  println!("b: {}", b);
  println!("c: {}", c);
}
