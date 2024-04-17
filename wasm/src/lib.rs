use js_sys::{Float64Array, Function, Uint8ClampedArray};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod perform_ray_tracing;
use perform_ray_tracing::perform_ray_tracing;

mod rng;
mod sphere;
mod vec;

fn to_int(x: f64) -> u8 {
    let f = clamp(x).powf(1. / 2.2) * 255. + 0.5;
    return f as u8;
}

fn clamp(x: f64) -> f64 {
    if x < 0. {
        0.
    } else if x > 1. {
        1.
    } else {
        x
    }
}

#[wasm_bindgen]
pub fn run(w: usize, h: usize, samps: u32, notify: &JsValue) -> f64 {
    let notify = Function::try_from(notify).unwrap();

    for y in 0..h {
        let res = perform_ray_tracing(w, h, samps, y);

        let image = Uint8ClampedArray::new_with_length((w * 4) as u32);

        for i in 0..w {
            let color = res[i];
            let s = (i * 4) as u32;
            image.set_index(s, to_int(color.x));
            image.set_index(s + 1, to_int(color.y));
            image.set_index(s + 2, to_int(color.z));
            image.set_index(s + 3, 255);
        }

        notify.call2(&JsValue::NULL, &JsValue::from_f64(y as f64), &image);
    }

    return 1.;

    // let cc = Float64Array::new_with_length(c.len() as u32);
    // cc.set_index(0, 2.);
    // cc.set_index(1, 5.);

    // let null = JsValue::null();

    // notify.call1(&null, &cc);

    // return vec![c[0] + 1., c[1] + 1.];
}
