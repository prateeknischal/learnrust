enum FileEventActionType {
  UNLINK
}

enum Number {
  ZERO,
  ONE,
  TWO,
}

type FE = FileEventActionType;

fn main() -> () {
  let _v:FE = FileEventActionType::UNLINK;

  // Can't use crate::FE
  use crate::FileEventActionType::UNLINK;

  let w = UNLINK;
  match w {
    _v => println!("They are found !!"),
    MOVED => println!("But why")
  }

  println!("Enum as number: {}", Number::ZERO as i32);
}
