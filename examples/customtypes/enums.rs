enum FileEvent {
  WRITE,
  DELETE
}

fn main() -> () {
  let ev:FileEvent = FileEvent::WRITE;
  match ev {
    FileEvent::WRITE => println!("Write event"),
    FileEvent::DELETE => println!("Delete event")
  }
}
