use std::cmp::min;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::mpsc::channel;
use std::thread::spawn;

fn frequency_single_thread(texts: Vec<String>) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                *map.entry(c).or_insert(0) += 1;
            }
        }
    }

    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let worker_count = min(worker_count, input.len());

    return match worker_count {
        0 => { HashMap::new() }
        1 => { frequency_single_thread(input.iter().map(|&s| String::from(s)).collect()) }
        _ => {
            // Setup
            let (tx, rx) = channel::<HashMap<char, usize>>();
            let senders = vec![tx; worker_count];
            let chunks = input.chunks(input.len().div_ceil(worker_count));

            // Create threads
            for (chunk, sender) in zip(chunks, senders) {
                let chunk: Vec<String> = chunk.iter().map(|&s| String::from(s)).collect();
                spawn(move || {
                    let thread_hm = frequency_single_thread(chunk);
                    sender.send(thread_hm).unwrap();
                });
            }

            // Collect results
            let mut hm = HashMap::new();
            for thread_hm in rx {
                for (&k, &v) in thread_hm.iter() {
                    *hm.entry(k).or_insert(0) += v;
                }
            }
            hm
        }
    };
}
