fn main() -> () {
  let mut lang:Vec<&str> = vec!["rust", "golang", "java", "python", "c++"];

  for v in 0..lang.len() {
    println!("Language {}: {}", v, lang[v]);
  }

  for v in lang.iter() {
    println!("Language: {}", v);
  }

  for v in lang.iter_mut() {
    *v = match v {
      &mut "golang" => "go",
      _ => v
    }
  }

  println!("{:?}", lang);

  let lang2:Vec<&str> = vec!["rust", "golang", "java", "python", "c++"];
  for v in lang2.into_iter() {
    println!("Reading out: {}", v);
  }

  // No longer available
  // println!("{:?}", lang2.len())
}
