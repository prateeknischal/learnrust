
struct UnPrintable(i32);

#[derive(Debug)]
struct Printable(i32);

fn main() -> () {
  println!("This is println from {}", "println!");
  eprint!("This is an eprint \n");

  println!("Name: {:>width$}", "Prateek", width=15);
  println!("Name: {0} {1} {lastname}", "Prateek", "Kumar", lastname="Nischal");

  let pi = 3.14159;
  let pi_str = format!("Value of pi: {:.3}", pi);
  println!("{}", pi_str);


  println!("printable: {:?}", Printable(42));
  // Errors out
  //println!("unprintable: {:#?}", UnPrintable(32));
}
