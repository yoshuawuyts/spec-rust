#[derive(Debug)]
pub struct Example {
  name: String,
}

/// Create an empty box.
///
/// ```
/// let myBox = spec::heap::create_empty();
/// assert_eq!(myBox.count(), 0);
/// ```
pub fn create_empty() -> Box<Example> {
  Box::new(Example {
    name: String::from("sup"),
  })
}
