extern crate spec;

use spec::reference_counters as refs;

fn main() {
  let res = refs::sup();
  println!("{:?}", res);
}
