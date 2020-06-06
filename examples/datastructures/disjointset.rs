struct DSU {
  par: Vec<usize>
}

impl DSU {
  fn union(&mut self, x: usize, y: usize) -> () {
    let _x = self.find(x);
    let _y = self.find(y);

    self.par[_x] = _y;
  }

  fn find(&mut self, x: usize) -> usize {
    if self.par[x] != x {
      self.par[x] = self.find(self.par[x]);
    }

    return self.par[x]
  }
}

fn main() -> () {
  let v: Vec<usize> = vec![0, 1, 1, 1, 2, 2, 3];
  let mut dsu = DSU{ par: v };
  println!("parent: {}", dsu.find(3));
  dsu.union(5, 3);

  println!("{}: {}", dsu.find(3), dsu.find(5));
}
