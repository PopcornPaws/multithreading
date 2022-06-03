use futures_channel::oneshot;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod pool;

pub type CharMap = HashMap<char, usize>;

pub fn frequency_in_string(input: String) -> CharMap {
    let mut map = CharMap::new();
    for c in input.chars().filter(|c| c.is_alphabetic()) {
        *map.entry(c.to_ascii_lowercase()).or_default() += 1;
    }
    map
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

#[wasm_bindgen]
pub struct Text {
    inner: String,
}

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(inner: String) -> Self {
        Self { inner }
    }

    pub fn process(self, concurrency: usize, pool: &pool::WorkerPool) {
        let mut map = CharMap::new();

        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(concurrency)
            .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
            .build()
            .unwrap();

        let (tx, rx) = oneshot::channel();
        pool.run(move || thread_pool.install(|| {}));
        drop(tx.send(todo!()));
        todo!();
    }
}

#[wasm_bindgen]
pub struct ProcessedText {
    promise: Promise,
    map: HashMap<char, usize>,
}
