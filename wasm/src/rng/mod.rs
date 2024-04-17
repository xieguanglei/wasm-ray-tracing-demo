use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/rand-arr.js")]
extern "C" {
    fn rand_arr(count: usize) -> Vec<f64>;
}


const SIZE: usize = 1000;

pub struct RngWrap {
    i: usize,
    arr: Vec<f64>,
}

impl RngWrap {
    pub fn new() -> Self {
        RngWrap {
            i: 0,
            arr: rand_arr(SIZE)
        }
    }

    pub fn gen(&mut self) -> f64 {

        self.i += 1;

        if self.i == SIZE {
            self.arr = rand_arr(SIZE);
            self.i = 0;
        }

        return self.arr[self.i];
    }
}
