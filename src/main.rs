use std::mem;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use wasmworkers::{frequency_in_string, CharMap};

fn frequency_single_threaded(input: &[&str]) -> CharMap {
    let mut map = CharMap::new();
    for line in input {
        for c in line.chars().filter(|c| c.is_alphabetic()) {
            *map.entry(c.to_ascii_lowercase()).or_default() += 1;
        }
    }

    map
}

fn frequency_multithreaded(input: &[&str], worker_count: usize) -> CharMap {
    let mut result = CharMap::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let handle = thread::spawn(move || frequency_in_string(string));
        handles.push(handle);
    }

    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_default() += value;
        }
    }

    result
}

fn frequency_channels(input: &[&str], worker_count: usize) -> CharMap {
    let mut result = CharMap::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let (sender, receiver) = mpsc::channel();
    for chunk in chunks {
        let sender = sender.clone();
        let string = chunk.join("");
        thread::spawn(move || {
            let map = frequency_in_string(string);
            sender.send(map).unwrap();
        });
    }
    mem::drop(sender);

    for received in receiver {
        for (key, value) in received {
            *result.entry(key).or_default() += value;
        }
    }
    result
}

fn frequency_mutex(input: &[&str], worker_count: usize) -> CharMap {
    let result = Arc::new(Mutex::new(CharMap::new()));
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let map = frequency_in_string(string);
            let mut result = result.lock().unwrap();
            for (key, value) in map {
                *result.entry(key).or_default() += value;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn main() {
    let data = &[
        "To be or Not to Be, that is the question. Said the guy named Hamlet before he set out to take revenge on his uncle."; 1024];

    let worker_count = 8;

    let sys_time = std::time::SystemTime::now();
    let _map = frequency_single_threaded(data);
    println!(
        "elapsed: {} micros",
        sys_time.elapsed().unwrap().as_micros()
    );

    let sys_time = std::time::SystemTime::now();
    let _map = frequency_multithreaded(data, worker_count);
    println!(
        "elapsed: {} micros",
        sys_time.elapsed().unwrap().as_micros()
    );

    let sys_time = std::time::SystemTime::now();
    let _map = frequency_channels(data, worker_count);
    println!(
        "elapsed: {} micros",
        sys_time.elapsed().unwrap().as_micros()
    );

    let sys_time = std::time::SystemTime::now();
    let _map = frequency_mutex(data, worker_count);
    println!(
        "elapsed: {} micros",
        sys_time.elapsed().unwrap().as_micros()
    );
}
