use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CoolNumber{
    value: u32
}

#[wasm_bindgen]
pub struct MyEmpty {
    value: Option<String>
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

#[wasm_bindgen]
impl MyEmpty {
    pub fn new() -> MyEmpty {
        MyEmpty { value: None }
    }

    pub fn return_four(&self) -> u32 {
        4
    }

    pub fn return_my_name(&self) -> String {
        String::from("julio")
    }
}

#[wasm_bindgen]
pub struct NewFancyStruct;