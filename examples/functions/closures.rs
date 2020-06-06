

// By Value
fn run<F>(foo: F) where F: FnOnce() {
  foo()
}

fn run_x<F>(mut foo: F) where F: FnMut(&str) {
  foo("qwerty")
}

fn run_y<F>(foo: F) where F: FnMut() -> () {
  foo()
}


fn main() -> () {
  let s:&str = "Hello  . world";
  let mut v:Vec<&str> = s.split(".").collect();

  for x in v.iter_mut() {
    *x = || -> &str { x.trim() }();
  }
  println!("{:?}", v);

  let t = "hello";
  let read_only = || println!("{}", t);
  run(read_only);

  let write_too = |p: &str| { p.to_ascii_uppercase(); };
  run_x(write_too);
  println!("{}", &t);

  let u: &mut str = "world";
  let append = || { u.make_ascii_uppercase(); };
  run_y(append);
}
