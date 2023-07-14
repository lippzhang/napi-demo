#![deny(clippy::all)]

use std::collections::HashMap;
use std::thread;
 
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode, ThreadSafeCallContext}, JsString, CallContext, JsUndefined,
};
 

use napi::Either;
use napi::Env;
use napi::JsFunction;
use napi::bindgen_prelude::Array;
use napi::bindgen_prelude::Float32Array;
use napi::bindgen_prelude::Null;
use napi::bindgen_prelude::Object;
use napi::bindgen_prelude::Uint32Array;
use napi::bindgen_prelude::Undefined;
use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiValue;


// exports
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub const DEFAULT_COST:u32 = 12;


#[napi(constructor)]
struct Animal {
  pub name: String,
  pub kind: u32,
}

#[napi]
impl Animal {
  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}

#[napi]
pub enum Kind {
  Duck,
  Dog,
  Cat,
}

// naming conventions
#[napi(js_name = "coolFunction")]
fn a_function(a_arg: u32) -> u32 {
  a_arg + 1
}

// values
#[napi]
fn get_undefined() -> Undefined {
	()
}
 
// default return or empty tuple `()` are `undefined` after converted into JS value.
#[napi]
fn log(n: u32) {
	println!("{}", n);
}

#[napi]
fn get_null() -> Null {
  Null
}

#[napi]
fn get_env(env: String) -> Option<String> {
  match std::env::var(env) {
    Ok(val) => Some(val),
    Err(val) => None,
  }
}

#[napi(object)]
pub struct TypeDemo {
  pub name: String,
}


#[napi]
fn read(value: Either<String, TypeDemo>) -> String {
  match value {
      Either::A(native) => native,
      Either::B(_value) => _value.name,
  }
}

#[napi(object)]
struct  fnObject {
  pub name: String,
  #[napi(ts_type = "(error, string) => string")]
  pub nameFn: JsFunction,
}

#[napi]
fn read2(value: Either<String, fnObject>) ->  Result<()> {
  match value {
      Either::A(native) =>  Ok(()),
      Either::B(_value) => {
        let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = _value.nameFn
        .create_threadsafe_function(0, |ctx| {
          ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
        })?;
      for n in 0..100 {
        let tsfn = tsfn.clone();
        thread::spawn(move || {
          tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
        });
      }
      Ok(())
      },
    }
}

#[napi]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}


#[napi]
fn is_good() -> bool {
	true
}

#[napi]
fn keys(obj: Object) -> Vec<String> {
  Object::keys(&obj).unwrap()
}

// #[napi] erro
// fn log_string_field(obj: Object, field: String) {
// 	println!("{}: {:?}", &field,obj.get::<String>::(field.as_ref()));
// }
#[napi]
fn create_obj(env: Env) -> Object {
	let mut obj = env.create_object().unwrap();
	obj.set("test", 1).unwrap();
	obj
}

/// #[napi(object)] requires all struct fields to be public
#[napi(object)]
struct PackageJson {
	pub name: String,
	pub version: String,
	pub dependencies: Option<HashMap<String, String>>,
	pub dev_dependencies: Option<HashMap<String, String>>,
}
 
#[napi]
fn log_package_name(package_json: PackageJson) {
	println!("name: {}", package_json.name);
}
 
#[napi]
fn read_package_json() -> PackageJson {
    let a = PackageJson {
      name: 'a'.to_string(),
      version: String::from("1.0"),
      dependencies: None,
      dev_dependencies: None,
    };
    a
}

#[napi]
fn arr_len(arr: Array) -> u32 {
  arr.len()
}
 
#[napi]
fn get_tuple_array(env: Env) -> Array {
  let mut arr = env.create_array(2).unwrap();
 
  arr.insert(1).unwrap();
  arr.insert("test").unwrap();
 
  arr
}
 
#[napi]
fn vec_len(nums: Vec<u32>) -> u32 {
  u32::try_from(nums.len()).unwrap()
}
 
#[napi]
fn get_nums() -> Vec<u32> {
  vec![1, 1, 2, 3, 5, 8]
}


#[napi]
fn convert_u32_array(input: Uint32Array) -> Vec<u32> {
  input.to_vec()
}
 
#[napi]
fn create_external_typed_array() -> Uint32Array {
  Uint32Array::new(vec![1, 2, 3, 4, 5])
}
 
#[napi]
fn mutate_typed_array(mut input: Float32Array) {
  for item in input.as_mut() {
    *item *= 2.0;
  }
}

// 写法好像已经废弃了，无法生成调用类型
// #[js_function(1)]
// fn not(ctx: CallContext) -> Result<JsBoolean> {
//   let condition: JsBoolean = ctx.get(0)?;
//   ctx.env.get_boolean(!condition.get_value()?)
// }
// #[js_function(1)]
// fn thread_safe_function(ctx: CallContext) -> Result<JsUndefined> {
//   let callback: JsFunction = ctx.get(0)?;
//   let tsfn = ctx.env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<u32>| {
//     ctx.env.create_uint32(ctx.value).map(|js_value| vec![js_value])
//   })?;
 
//   std::thread::spawn(move || {
//     tsfn.call(Ok(1), ThreadsafeFunctionCallMode::NonBlocking);
//   });
 
//   ctx.env.get_undefined()
// }
#[napi]
pub struct PathData {
  pub chunks: Vec<String>,
  pub name: String,
  pub age: u8,
}

#[napi]
pub fn call_threadsafe_function2(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<PathData, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<PathData>| {
    println!("rust ctx.value.name is {}", ctx.value.name);
     Ok(vec![ctx.value])
    //  Ok(vec![ctx.env.create_string("hello world")])
      // ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
    // tsfn.call(Ok("lippzhang".to_string()), ThreadsafeFunctionCallMode::Blocking);
    // tsfn.call(Err(Error::new(Status::InvalidArg, "参数错误")), ThreadsafeFunctionCallMode::Blocking);
    let fromJsValue = tsfn.call(Ok(PathData {
      chunks: vec!["123".to_string(), "444".to_string()],
      name: String::from("lippzhang"),
      age: 1
    }), ThreadsafeFunctionCallMode::Blocking);
    Ok(())
}