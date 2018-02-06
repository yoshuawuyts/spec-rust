pub fn create_vector() -> Vec<i32> {
  Vec::new()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn initialize_an_empty_vector() {
    let vec = create_vector();
    assert_eq!(vec.len(), 0);
  }
}
