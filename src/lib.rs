use futures_channel::oneshot;
use js_sys::Promise;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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

#[allow(dead_code)]
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

    pub fn process(
        self,
        concurrency: usize,
        input: String,
        pool: &pool::WorkerPool,
    ) -> Result<Promise, JsValue> {
        let n_chunks = (input.len() / concurrency).max(1);
        let chunkies = input
            .as_bytes()
            .chunks(n_chunks)
            .flat_map(|chunk| String::from_utf8(chunk.to_vec()))
            .collect::<Vec<String>>();

        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(concurrency)
            .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
            .build()
            .unwrap();

        let (tx, rx) = oneshot::channel();
        pool.run(move || {
            thread_pool.install(|| {
                let map: CharMap = chunkies
                    .par_iter()
                    .fold(CharMap::new, |mut acc, chunk| {
                        for c in chunk.chars() {
                            *acc.entry(c).or_default() += 1;
                        }
                        acc
                    })
                    .reduce_with(|mut m1, m2| {
                        for (k, v) in m2 {
                            *m1.entry(k).or_default() += v;
                        }
                        m1
                    })
                    .unwrap();
                drop(tx.send(map));
            });
        })?;

        // todo turn this into a future/promise
        let done = async move {
            match rx.await {
                Ok(map) => JsValue::from_serde(&map).map_err(|e| e.to_string().into()),
                Err(_) => Err(JsValue::undefined()),
            }
        };

        Ok(wasm_bindgen_futures::future_to_promise(done))
    }
}
