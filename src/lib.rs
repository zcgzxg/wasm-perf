use std::vec;

use base64::{engine::general_purpose, Engine};
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bench(items: usize, item_bytes: usize) -> usize {
    let mut result = Vec::with_capacity(items);

    for _ in 0..items {
        let mut item = vec![0_u8; item_bytes];
        thread_rng().fill(item.as_mut_slice());
        result.push(general_purpose::STANDARD.encode(item));
    }

    result
        .iter()
        .map(|it| it.len())
        .fold(0, |acc, len| acc + len)
}
