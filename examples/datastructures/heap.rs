struct Heap {
  h: Vec<i32>,
}

impl Heap {
  fn insert(&mut self, v: i32) -> () {
    self.h.push(v);
    let mut idx: usize = self.h.len() - 1;
    while idx >= 1 {
      let par: usize = idx / 2;
      if self.h[par] > v {
        self.h[idx] = self.h[par];
        self.h[par] = v;
      }

      idx = par;
    }
  }

  fn peek(&mut self) -> i32 {
    self.h[0]
  }

  fn top(&mut self) -> i32 {
    let ret = self.h[0];
    let last = self.h.pop().unwrap();
    if self.h.len() == 0 {
      return ret;
    }

    self.h[0] = last;

    let mut idx: usize = 0;
    let sz: usize = self.h.len();
    while idx < sz {
      let mut l = idx * 2 + 1;
      let r = idx * 2 + 2;

      if l < sz && r < sz {
        if self.h[l] > self.h[r] {
          l = r;
        }
      }

      if l < sz && self.h[idx] > self.h[l] {
        let t = self.h[idx];
        self.h[idx] = self.h[l];
        self.h[l] = t;
        idx = l;
      } else {
        break
      }
    }
    ret
  }
}

fn main() -> () {
  let mut heap = Heap{ h: vec![] };

  for &i in [4, 1, 2, 5, 3, 2, 6].iter() {
    heap.insert(i);
  }

  println!("Top of the heap: {}", heap.peek());

  while heap.h.len() > 0 {
    print!("{} ", heap.top());
  }
  println!("");
}
