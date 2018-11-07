// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/data.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 5
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 1)
      (data (i32.const 0) \"\")
      (data (i32.const 1) \"abcd\")
      (data (i32.const 0) \"\")
      (data (i32.const 0) \"abc\")
      (data (i32.const 0) \"\")
      (data (i32.const 1) \"abcd\")
      (data (i32.const 0) \"\")
      (data (i32.const 0) \"abc\")
      (data (i32.const 0) \"\")
      (data (i32.const 1) \"abcd\")
      (data (i32.const 0) \"\")
      (data (i32.const 0) \"abc\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 23

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 1)
      (data (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 27

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    // We group the calls together
    start_module_2(&result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 32

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    // We group the calls together
    start_module_3(&result_object);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 1)
      (data (i32.const 0) \"a\")
      (data (i32.const 3) \"b\")
      (data (i32.const 100) \"cde\")
      (data (i32.const 5) \"x\")
      (data (i32.const 3) \"c\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_4(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 40

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    // We group the calls together
    start_module_4(&result_object);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (i32.const 0) \"a\")
      (data (i32.const 1) \"b\")
      (data (i32.const 2) \"cde\")
      (data (i32.const 3) \"f\")
      (data (i32.const 2) \"g\")
      (data (i32.const 1) \"h\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_5(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 82

#[test]
fn test_module_5() {
    let result_object = create_module_5();
    // We group the calls together
    start_module_5(&result_object);
}
fn create_module_6() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 1)
      (data (i32.const 0) \"a\")
      (data (i32.const 65535) \"b\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_6(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 87

#[test]
fn test_module_6() {
    let result_object = create_module_6();
    // We group the calls together
    start_module_6(&result_object);
}
fn create_module_7() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 1))
      (data (i32.const 0) \"a\")
      (data (i32.const 65535) \"b\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_7(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 93

#[test]
fn test_module_7() {
    let result_object = create_module_7();
    // We group the calls together
    start_module_7(&result_object);
}
fn create_module_8() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 2)
      (data (i32.const 131071) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_8(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 98

#[test]
fn test_module_8() {
    let result_object = create_module_8();
    // We group the calls together
    start_module_8(&result_object);
}
fn create_module_9() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 0)
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_9(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 102

#[test]
fn test_module_9() {
    let result_object = create_module_9();
    // We group the calls together
    start_module_9(&result_object);
}
fn create_module_10() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_10(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 107

#[test]
fn test_module_10() {
    let result_object = create_module_10();
    // We group the calls together
    start_module_10(&result_object);
}
fn create_module_11() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 0 0)
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_11(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 112

#[test]
fn test_module_11() {
    let result_object = create_module_11();
    // We group the calls together
    start_module_11(&result_object);
}
fn create_module_12() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 1)
      (data (i32.const 65536) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_12(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 117

#[test]
fn test_module_12() {
    let result_object = create_module_12();
    // We group the calls together
    start_module_12(&result_object);
}
fn create_module_13() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 0)
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_13(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 121

#[test]
fn test_module_13() {
    let result_object = create_module_13();
    // We group the calls together
    start_module_13(&result_object);
}
fn create_module_14() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_14(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 126

#[test]
fn test_module_14() {
    let result_object = create_module_14();
    // We group the calls together
    start_module_14(&result_object);
}
fn create_module_15() -> ResultObject {
    let module_str = "(module
      (memory (;0;) 0 0)
      (data (i32.const 0) \"\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_15(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 131

#[test]
fn test_module_15() {
    let result_object = create_module_15();
    // We group the calls together
    start_module_15(&result_object);
}
fn create_module_16() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_16(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 136

#[test]
fn test_module_16() {
    let result_object = create_module_16();
    // We group the calls together
    start_module_16(&result_object);
}
fn create_module_17() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0 3))
      (data (i32.const 0) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_17(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 155

#[test]
fn test_module_17() {
    let result_object = create_module_17();
    // We group the calls together
    start_module_17(&result_object);
}
fn create_module_18() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0))
      (data (i32.const 1) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_18(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 160

#[test]
fn test_module_18() {
    let result_object = create_module_18();
    // We group the calls together
    start_module_18(&result_object);
}
fn create_module_19() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"memory\" (memory (;0;) 0 3))
      (data (i32.const 1) \"a\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_19(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 168

// Line 176

// Line 184

// Line 192

// Line 200

// Line 217

// Line 226

// Line 233

// Line 241

// Line 249

// Line 257

// Line 264

// Line 272

// Line 279

// Line 289
#[test]
fn c33_l289_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 11, 6, 1, 0, 65, 0, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 298
#[test]
fn c34_l298_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 6, 1, 0, 66, 0, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 306
#[test]
fn c35_l306_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 65, 0, 104, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 314
#[test]
fn c36_l314_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 5, 1, 0, 1, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 322
#[test]
fn c37_l322_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 1, 65, 0, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 330
#[test]
fn c38_l330_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 5, 3, 1, 0, 1, 11, 7, 1, 0, 65, 0, 1, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_19() {
    let result_object = create_module_19();
    // We group the calls together
    start_module_19(&result_object);
}
