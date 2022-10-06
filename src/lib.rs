use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CoolNumber(u64);

#[wasm_bindgen]
impl CoolNumber {
    pub fn new(val: u64) -> CoolNumber {
        CoolNumber(val)
    }
}
