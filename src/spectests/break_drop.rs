// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/break_drop.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        block  ;; label = @1
          br 0 (;@1;)
        end)
      (func (;1;) (type 0)
        block  ;; label = @1
          i32.const 1
          br_if 0 (;@1;)
        end)
      (func (;2;) (type 0)
        block  ;; label = @1
          i32.const 0
          br_table 0 (;@1;)
        end)
      (export \"br\" (func 0))
      (export \"br_if\" (func 1))
      (export \"br_table\" (func 2)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 7
fn c1_l7_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c1_l7_action_invoke");
    let func_index = match result_object.module.info.exports.get("br") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, ());
}

// Line 8
fn c2_l8_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c2_l8_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_if") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, ());
}

// Line 9
fn c3_l9_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c3_l9_action_invoke");
    let func_index = match result_object.module.info.exports.get("br_table") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, ());
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
    c1_l7_action_invoke(&result_object);
    c2_l8_action_invoke(&result_object);
    c3_l9_action_invoke(&result_object);
}
