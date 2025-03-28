mod xfast_trie;

use csv::Writer;
use rand::Rng;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};
use xfast_trie::XFastTrie;

fn create_csv_file(file_path: &str, headers: &[&str]) -> Result<(), Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        let mut wtr = Writer::from_path(file_path)?;
        wtr.write_record(headers)?;
        wtr.flush()?;
    }
    Ok(())
}

fn save_results() -> Result<(), Box<dyn Error>> {
    create_csv_file(
        "results.csv",
        &[
            "Data Structure",
            "Universe Size",
            "# of Values",
            "Time",
            "Memory",
        ],
    );
    let mut wtr = Writer::from_path("results.csv")?;
    wtr.write_record(&["X-Fast Trie", "2^8", "2^7", "time", "memory"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    //let mut rng = rand::thread_rng();
    //let mut random_values = HashSet::new();

    //while random_values.len() < 85000000 {
    //    random_values.insert(rng.gen::<u32>());
    // }

    //let start = Instant::now();
    //let mut duration = start.elapsed();

    let mut test: XFastTrie<u32> = XFastTrie::new();
    for value in (1..50).step_by(5) {
        test.insert(value, value);
    }
    for value in (2147483600..2147483647).step_by(5) {
        test.insert(value, value);
    }

    for value in (1..50).step_by(1) {
        println!(
            "Value: {}, Predecessor value: {}",
            value,
            test.predecessor(&value).unwrap()
        );
    }
    for value in (2147483599..2147483647).step_by(1) {
        println!(
            "Value: {}, Predecessor value: {}",
            value,
            test.predecessor(&value).unwrap()
        );
    }
}
