/// Create a new vector.
///
/// ```
/// let vec = spec::heap::create_empty();
/// assert_eq!(vec.len(), 0);
/// ```
pub fn create_empty() -> Vec<i32> {
  Vec::new()
}
