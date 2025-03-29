mod xfast_trie;

use csv::Writer;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::time::Instant;
use xfast_trie::XFastTrie;

fn save_results(record: &[&str; 5]) -> Result<(), Box<dyn Error>> {
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
            "Time",
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
        let random_number: u32 = rng.random();
        if random_number < max {
            random_values.insert(random_number);
        }
    }
    let mut result: Vec<u32> = random_values.into_iter().collect();
    result.sort();
    result
}

fn main() {
    // inputs
    let input_size = 1000000;
    let universe_size = "2^32";

    // Setup
    let mut xfast: XFastTrie<u32> = XFastTrie::new();
    let mut btree: BTreeMap<u32, u32> = BTreeMap::new();
    let input_data1: Vec<u32> = create_data(input_size, 2147483647);
    let input_data2: Vec<u32> = input_data1.clone();

    // X-Fast Trie
    let mut start = Instant::now();
    for value in input_data1 {
        xfast.insert(value, value);
    }
    save_results(&[
        "X-Fast Trie",
        universe_size,
        &input_size.to_string(),
        "Insert",
        &start.elapsed().as_millis().to_string(),
    ])
    .expect("ERROR");

    start = Instant::now();
    for value in 1..input_size {
        xfast.predecessor(&value);
    }
    save_results(&[
        "X-Fast Trie",
        universe_size,
        &input_size.to_string(),
        "Predecessor",
        &start.elapsed().as_millis().to_string(),
    ])
    .expect("ERROR");

    start = Instant::now();
    for value in 1..input_size {
        xfast.get(&value);
    }
    save_results(&[
        "X-Fast Trie",
        universe_size,
        &input_size.to_string(),
        "Get",
        &start.elapsed().as_millis().to_string(),
    ])
    .expect("ERROR");

    start = Instant::now();
    for value in input_data2 {
        btree.insert(value, value);
    }
    save_results(&[
        "B Tree",
        universe_size,
        &input_size.to_string(),
        "Insert",
        &start.elapsed().as_millis().to_string(),
    ])
    .expect("ERROR");

    start = Instant::now();
    for value in 1..input_size {
        btree.get(&value);
    }
    save_results(&[
        "B Tree",
        universe_size,
        &input_size.to_string(),
        "Get",
        &start.elapsed().as_millis().to_string(),
    ])
    .expect("ERROR");
}
