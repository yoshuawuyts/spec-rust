use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub struct Example {
  pub name: String,
}

impl Iterator for Example {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    None
  }
}

/// Create an empty box.
///
/// ```rust
/// let myBox = spec::heap::create_empty();
/// assert_eq!(*myBox, spec::heap::Example { name: String::from("sup")});
/// ```
pub fn create_empty() -> Box<Example> {
  Box::new(Example {
    name: String::from("sup"),
  })
}
