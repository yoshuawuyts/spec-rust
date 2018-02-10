// /// ```rust
// /// #[macro_use] extern crate spec;
// /// spec::macros::foo!(y => 3);
// /// ```

// macro_rules! foo {
//   (x => $e:expr) => (println!("mode x {}", $e));
//   (y => $e:expr) => (println!("mode x {}", $e));
// }
