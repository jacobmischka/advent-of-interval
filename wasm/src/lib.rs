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
pub fn day_1(input: &str) -> Vec<i32> {
    let results = days::day_01::main(input);
    ctx_log(&format!("Logging! {:?}", results));
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_2(input: &str) -> Vec<i32> {
    let results = days::day_02::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_3(input: &str) -> Vec<i32> {
    let results = days::day_03::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_4(input: &str) -> Vec<i32> {
    let results = days::day_04::main(input);
    vec![results.0 as _, results.1 as _]
}

#[wasm_bindgen]
pub fn day_5(input: &str) -> Vec<JsValue> {
    let results = days::day_05::main(input);
    vec![results.0.into(), results.1.into()]
}
