//! Trying out reference counters
/// https://doc.rust-lang.org/book/second-edition/ch15-04-rc.html

use std::rc::Rc;

#[derive(Debug)]
pub struct Foo {
  pub bar: Vec<Rc<i32>>,
}

pub fn sup() -> (Vec<Rc<i32>>, Foo) {
  let num: i32 = 12;
  let a = Rc::new(num);

  let mut y = Vec::new();
  y.push(Rc::clone(&a));

  let mut x = Vec::new();
  x.push(Rc::clone(&a));
  let structure = Foo { bar: x };

  (y, structure)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rc() {
    sup();
  }
}
