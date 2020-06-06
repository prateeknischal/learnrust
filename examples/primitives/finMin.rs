use std::mem;

fn min(a: &[i32]) -> i32 {
  let mut min = a[0];
  for (_, &x) in a.iter().enumerate() {
    if x < min {
      min = x;
    }
  }

  return min
}

fn main() -> (){
  let a: [i32; 5] = [1, 2, -3, 4, 5];
  println!("Min: {}", min(&a));
  println!("Size of the array: {}", mem::size_of_val(&a));
  println!("Min of a slice: {}", min(&a[1..2]));
}
