#![feature(proc_macro, generators)]

#[macro_use] extern crate failure;
#[macro_use] extern crate futures_await as futures;
extern crate tokio;

use futures::prelude::*;
use failure::Error;
use std::path;
use tokio::{fs, io::AsyncRead, runtime::Runtime};

fn main () -> Result<(), Error>{
  let mut eloop = Runtime::new()?;

  // NOTE: how can we make the main thread exit on error here?
  eloop.spawn(read_file().map_err(|err| println!("{:?}", err)));

  // NOTE: the `Error` type here is defined as `()`. This means we can't use `?`
  eloop.shutdown_on_idle().wait().unwrap();
  Ok(())
}

#[async]
fn read_file () -> Result<(), Error> {
  let file = path::Path::new("./README.md");
  let file = await!(fs::file::File::open(file))?;

  // TODO: so this super weird syntax over here where it owns, then returns the
  // owned value is because we can't do borrows with async/await.
  // This is isn't great - but not having this would be *so much worse*. So
  // yeah, we just have to accept the reality we live in. I like it.
  let (file, data) = await!(tokio::io::read_to_end(file, vec![]))?;
  println!("\n{:#}", String::from_utf8(data)?);
  Ok(())
}
