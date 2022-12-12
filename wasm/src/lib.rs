mod utils;

use adventofcode_2022::days;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
    fn ctx_log(s: &str);
}

#[wasm_bindgen]
pub fn day_01(input: &str) -> Vec<i32> {
    let results = days::day_01::main(input);
    ctx_log(&format!("Logging! {:?}", results));
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_02(input: &str) -> Vec<i32> {
    let results = days::day_02::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_03(input: &str) -> Vec<i32> {
    let results = days::day_03::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_04(input: &str) -> Vec<i32> {
    let results = days::day_04::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_05(input: &str) -> Vec<JsValue> {
    let results = days::day_05::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_06(input: &str) -> Vec<JsValue> {
    let results = days::day_06::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_07(input: &str) -> Vec<JsValue> {
    let results = days::day_07::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_08(input: &str) -> Vec<JsValue> {
    let results = days::day_08::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_09(input: &str) -> Vec<JsValue> {
    let results = days::day_09::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_10(input: &str) -> Vec<JsValue> {
    let results = days::day_10::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_11(input: &str) -> Vec<JsValue> {
    let results = days::day_11::main(input);
    vec![results.0.into(), results.1.into()]
}

#[wasm_bindgen]
pub fn day_12(input: &str) -> Vec<JsValue> {
    let results = days::day_12::main(input);
    vec![results.0.into(), results.1.into()]
}
