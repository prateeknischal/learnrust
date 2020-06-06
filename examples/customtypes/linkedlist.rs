use crate::List::*;

enum List {
  Pair(u32, Box<List>),
  Nil,
}

// Attach methods to enum

impl List {
  fn new() -> List {
    return Nil;
  }

  fn add(self, e: u32) -> List {
    return Pair(e, Box::new(self));
  }

  fn len(&self) -> u32 {
    match *self {
      Pair(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }

  fn stringify(&self) -> String {
    match *self {
      Pair(head, ref tail) => {
        format!("{}, {}", head, tail.stringify())
      },
      Nil => {
        format!("Nil")
      }
    }
  }
}

fn main() -> () {
  let mut list = List::new();
  list = list.add(3);
  list = list.add(2);
  list = list.add(1);


  println!("Length {}", list.len());
  println!("{}", list.stringify());

  // Can't assign a value multiple times
  let mut later;

  later = 5;
  later = 10;

  println!("later: {}", later);
}
