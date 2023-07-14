#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::uninlined_format_args)]

use napi::{Env, JsUnknown};

#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "snmalloc")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[napi]
/// This is a const
pub const DEFAULT_COST: u32 = 12;

#[napi(skip_typescript)]
pub const TYPE_SKIPPED_CONST: u32 = 12;

mod test;
mod tsf;

#[napi]
pub fn run_script(env: Env, script: String) -> napi::Result<JsUnknown> {
  env.run_script(script)
}
