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

    pub fn process(self, concurrency: usize, input: String, pool: &pool::WorkerPool) -> Result<ProcessedText, JsValue> {
        let mut map = CharMap::new();
        let n_chunks = (input.len() / concurrency).max(1);

        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(concurrency)
            .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
            .build()
            .unwrap();

        let (tx, rx) = oneshot::channel();
        pool.run(move || {
            thread_pool.install(|| {
                input.chars().par_chunks(n_chunks)
                    .for_each(|chunk| {
                        let string = chunk.join("");
                        for c in input.chars().filter(|c| c.is_alphabetic()) {
                            *map.entry(c.to_ascii_lowercase()).or_default() += 1;
                        }
                    })
            });
            drop(tx.send(map));
        })?;

        let done = async move {
            match rx.await {
                Ok(data) => Ok(data.into()),
                Err(_) => Err(JsValue::undefined()),
            }
        };

        Ok(ProcessedText {
            promise: wasm_bindgen_futures::future_to_promise(done),
        })
    }
}

#[wasm_bindgen]
pub struct ProcessedText {
    promise: Promise,
}
