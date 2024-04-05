#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{Null, Object, Undefined},
  Env,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn keys(obj: Object) -> Vec<String> {
  Object::keys(&obj).unwrap()
}

#[napi]
fn log_string_field(obj: Object, field: String) {
  println!("{}: {:?}", &field, obj.get::<String>::(field.as_ref()));
}
