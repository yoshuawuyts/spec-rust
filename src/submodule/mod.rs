//! Example submodule with internal modules.

mod internal;

/// calls internal::print();
pub fn print() {
  internal::print();
}
