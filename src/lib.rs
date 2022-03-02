#![deny(clippy::all)]

use glob::glob;
use napi::bindgen_prelude::*;
use std::fs;

// using features and tokio_fs for working fs non-blocking
// use futures::prelude::*;
// use napi::tokio::fs;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Jasat {
  pub source: String,
  pub content: String,
}

#[napi(js_name = "glob")]
pub async fn glob_(path: String) -> Result<Vec<Jasat>> {
  Ok(
    glob(&path.to_string())
      .unwrap()
      .map(|s| Jasat {
        source: s.as_ref().unwrap().display().to_string(),
        content: fs::read_to_string(s.unwrap().display().to_string())
          .unwrap()
          .to_string(),
      })
      .collect::<Vec<Jasat>>(),
  )
}
