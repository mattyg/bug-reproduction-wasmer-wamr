//! A Wasm module can import entities, like functions, memories,
//! globals and tables.
//!
//! This example illustrates how to use imported functions. They come
//! in 2 flavors:
//!
//!   1. Dynamic functions, where parameters and results are of a
//!      slice of `Value`,
//!   2. Native function, where parameters and results are statically
//!      typed Rust values.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example imported-function --release --features "cranelift"
//! ```
//!
//! Ready?

use wasmer::{
  imports, wat2wasm, Function, FunctionEnv, FunctionEnvMut, Instance, Module,
  Store, TypedFunction,
};


static STATIC_CONTEXT_VAL: i32 = 1234;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Declare WASM Module
  let wasm_bytes = wat2wasm(
      br#"
(module
  (func $multiply_typed 
      (import "env" "multiply_typed") 
      (param i32) 
      (result i32)
  )
  (type $sum_t 
    (func (param i32) (param i32) (result i32))
  )
  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    (call $multiply_typed 
      (local.get $y)
    )
  )
(export "sum" (func $sum_f)))
"#,
  )?;
  let mut store = Store::default();
  struct MyEnv {}
  let env = FunctionEnv::new(&mut store, MyEnv {});
  let module = Module::new(&store, wasm_bytes)?;

  // Define some context data that the host function closure will use
  let context_val = 1234;
  fn my_val() -> i32 {
    1234
  }

  // Define the host function closure
  let multiply_by_3 = move |_env: FunctionEnvMut<MyEnv>, a: i32| -> i32 {
    println!("Expected STATIC_CONTEXT_VAL to equal 1234, actually equals {}", STATIC_CONTEXT_VAL);
    println!("Expected context_val to equal 1234, actually equals {}", context_val);
    println!("Expected my_val() to equal 1234, actually equals {}", my_val());

    a * 3
  };

  // Define the host function and WASM instance
  let multiply_typed = Function::new_typed_with_env(&mut store, &env, multiply_by_3);
  let import_object = imports! {
      "env" => {
          "multiply_typed" => multiply_typed,
      }
  };
  let instance = Instance::new(&mut store, &module, &import_object)?;

  // Execute the WASM function 'sum'
  let sum: TypedFunction<(i32, i32), i32> =
      instance.exports.get_function("sum")?.typed(&mut store)?;
  let result = sum.call(&mut store, 1, 2)?;
  assert_eq!(result, 6);

  Ok(())
}
