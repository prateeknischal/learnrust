use std::convert::TryInto;

fn pow_mod(a: i64, n: i64, m: i64) -> i64 {
  let mut res = 1i64;
  let mut d = a;
  let mut p = n;
  while p > 0 {
    if p & 1 == 1 {
      res = (res * d) % m;
    }

    p >>= 1;
    d = (d * d) % m;
  }
  return res
}

fn primes(n: i32) -> Vec<i32> {
  let mut p: Vec<i32> = vec![0;n as usize];
  p[1] = 1; p[2] = 1;
  for v in p.iter_mut().step_by(2) {
    *v = 2;
  }

  for i in (3usize .. n as usize).step_by(2) {
    if p[i] != 0 {
      continue
    }
    p[i] = i as i32;
    for j in (i * i .. n as usize).step_by(2usize * i as usize) {
      p[j] = i as i32;
    }
  }

  return p
}

fn factor(lp: &Vec<i32>, n: i32) -> Vec<(i32, i32)> {
  assert_eq!(n < lp.len().try_into().unwrap(), true);
  let mut f = Vec::<(i32, i32)>::new();
  let mut k = n;

  while k > 1 {
    let pr = lp[k as usize];
    let mut c = 0;
    while k > 1 && k % pr == 0 {
      c += 1;
      k = k / pr;
    }

    f.push((pr, c));
  }

  return f
}

fn main() -> () {
  // test pow_mod
  assert_eq!(pow_mod(5, 6, 7), 1i64);
  let pr = primes(1000);
  println!("{:?}", factor(&pr, 162));
}
