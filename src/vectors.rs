/// Create an empty vector.
///
/// ```rust
/// let vec = spec::vectors::create_empty();
/// assert_eq!(vec.len(), 0);
/// ```
pub fn create_empty() -> Vec<i32> {
  Vec::new()
}
