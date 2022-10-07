use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CoolNumber{
    value: u32
}

#[wasm_bindgen]
impl CoolNumber {
    pub fn new(value: u32) -> CoolNumber {
        CoolNumber { value }
    }

    pub fn get_val(&self) -> u32 {
        self.value
    }
}
