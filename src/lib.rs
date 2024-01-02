use base64::{engine::general_purpose, Engine};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn generate_b64(iterations: usize, b64_length: usize) -> usize {
    let mut results_b64 = Vec::with_capacity(iterations);
    let mut raw_bytes: Vec<u8> = vec![0; b64_length];

    for _ in 0..iterations {
        let data = raw_bytes.as_mut_slice();
        thread_rng().fill(data);
        results_b64.push(general_purpose::STANDARD.encode(data));
    }

    results_b64
        .iter()
        .map(|it| it.len())
        .fold(0, |acc, len| acc + len)
}

#[wasm_bindgen]
pub fn bench(iterations: usize, b64_length: usize) -> usize {
    if cfg!(target_arch = "wasm32") {
        console_error_panic_hook::set_once();
        let _ = console_log::init();
    }

    (0..iterations)
        .into_par_iter()
        .map(|_| generate_b64(1, b64_length))
        .sum::<usize>()
}
