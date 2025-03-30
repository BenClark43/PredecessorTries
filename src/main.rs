mod xfast_trie;

use csv::Writer;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::time::Instant;
use xfast_trie::XFastTrie;

fn save_results(record: &[&str; 6]) -> Result<(), Box<dyn Error>> {
    let file_exists = Path::new("results.csv").exists();
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("results.csv")?;
    let mut wtr = Writer::from_writer(&mut file);
    if !file_exists {
        wtr.write_record(&[
            "Data Structure",
            "Universe Size",
            "Input Size",
            "Operation",
            "Time (µs)",
            "Memory (Bytes)"
        ])?;
    }
    wtr.write_record(record)?;
    wtr.flush()?;
    Ok(())
}

fn create_data(amount: u32, max: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    let mut random_values = HashSet::new();

    while random_values.len() < amount as usize {
        let random_number: f32 = rng.random();

        random_values.insert((1f32 + random_number * (max - 1) as f32) as u32);
    }
    let mut result: Vec<u32> = random_values.into_iter().collect();
    result.sort();
    result
}

fn main() {
    // inputs
    let input_size = 4096;
    let universe_size = 262144 *2 *2 *2 *2*2*2*2*2;

    let mut tries: Vec<XFastTrie<u32>> = Vec::with_capacity(10);
    let mut data: Vec<Vec<u32>> = Vec::with_capacity(10);

    let mut insert_time = 0;
    let mut pred_time = 0;
    let mut memory = 0;

    for _ in 0..10 {
        tries.push(XFastTrie::new());
        data.push(create_data(input_size, universe_size));
    }
    for index in 0..10 {
        let input_data = data.get_mut(index).unwrap();
        let mut xfast = tries.get_mut(index).unwrap();
        let mut start = Instant::now();
        for value in input_data {
            xfast.insert(*value, *value);
        }
        insert_time += start.elapsed().as_micros();
        start = Instant::now();
        for value in 1..universe_size {
            xfast.predecessor(&value);
        }
        pred_time += start.elapsed().as_micros() * 1000 / universe_size as u128;
        memory += tries.get(0).unwrap().total_memory_usage();
    }
    insert_time = insert_time / 10;
    pred_time = pred_time / 10;
    memory = memory / 10;

    save_results(&[
        "X-Fast Trie",
        &universe_size.to_string(),
        &input_size.to_string(),
        "Insert",
        &insert_time.to_string(),
        &memory.to_string(),
    ]).expect("ERROR");

    save_results(&[
        "X-Fast Trie",
        &universe_size.to_string(),
        &input_size.to_string(),
        "Predecessor (1000)",
        &pred_time.to_string(),
        &memory.to_string(),
    ]).expect("ERROR");

    println!("END OF PROCESSING");
}
