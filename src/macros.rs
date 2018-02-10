/// Arm matching macro.
///
/// ```rust
/// #[macro_use] extern crate spec;
/// fn main() {
///   let res = foo!(x => 3);
///   assert_eq!(res, 3);
///
///   let res = foo!(y => 3);
///   assert_eq!(res, 6);
/// }
/// ```
#[macro_export]
macro_rules! foo {
  (x => $e:expr) => ($e);
  (y => $e:expr) => (2 * $e);
}

/// Multiply macro.
///
/// ```rust
/// #[macro_use] extern crate spec;
/// fn main() {
///   assert_eq!(multiply!(2 + 3), 25);
/// }
/// ```
#[macro_export]
macro_rules! multiply {
  ($e:expr) => ($e * 5);
}
