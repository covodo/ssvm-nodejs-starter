use wasm_bindgen::prelude::*;

fn is_prime(n: i32) -> bool {
  if n == 2 || n == 3 {
    return true;
  }

  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  let mut divisor: i32 = 6;
  while divisor * divisor - 2 * divisor + 1 <= n
  {
      if n % (divisor - 1) == 0 {
        return false;
      }
      if n % (divisor + 1) == 0 {
        return false;
      }
      divisor += 6;
  }

  return true;
}

#[wasm_bindgen]
pub fn next_prime(s: &str) -> String {
  println!("The Rust function next_prime() received {}", s);
  let mut n = s.parse().unwrap();

  n += 1;
  while !is_prime(n)
  {
    n += 1;
  }

  return n.to_string();
}