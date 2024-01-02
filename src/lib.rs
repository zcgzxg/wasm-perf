use base64::{engine::general_purpose, Engine};
use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_thread as thread;

#[cfg(target_arch = "wasm32")]
fn cpus() -> usize {
    use web_sys::window;

    window()
        .map(|w| w.navigator().hardware_concurrency() as usize)
        .unwrap_or(8)
}

#[cfg(not(target_arch = "wasm32"))]
fn cpus() -> usize {
    thread::available_parallelism()
        .map(|it| it.get())
        .unwrap_or(8)
}

#[wasm_bindgen(start)]
fn start() {}

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

    let mut iterations = iterations;
    let num_threads = cpus();
    let per_thread_items = if iterations % num_threads == 0 {
        iterations / num_threads
    } else {
        iterations / num_threads + 1
    };

    let (sender, receiver) = channel();
    for _i in 0..num_threads {
        let send = sender.clone();
        thread::spawn(move || {
            send.send(generate_b64(
                usize::min(per_thread_items, iterations),
                b64_length,
            ))
            .unwrap();
        });
        iterations -= per_thread_items;
    }

    let mut result: usize = 0;
    for _i in 0..num_threads {
        result += receiver.recv().unwrap();
    }
    result
}
