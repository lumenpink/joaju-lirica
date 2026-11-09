use fst::MapBuilder;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "scripts/lexicon_entries.tsv".to_string());
    let output_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "src/morphology/data/lexicon_pt.fst".to_string());

    let file = File::open(&input_path)?;
    let reader = BufReader::new(file);

    let mut map = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        let word = parts[0].trim().to_lowercase();
        let mask: u64 = parts[1].trim().parse()?;
        *map.entry(word).or_insert(0u64) |= mask;
    }

    println!("Total unique words loaded: {}", map.len());

    let out_file = File::create(&output_path)?;
    let mut builder = MapBuilder::new(BufWriter::new(out_file))?;

    for (word, mask) in map {
        builder.insert(word.as_bytes(), mask)?;
    }

    builder.finish()?;
    println!("Compiled FST to {}", output_path);

    let meta = std::fs::metadata(&output_path)?;
    println!(
        "Binary size: {} bytes ({:.2} MB)",
        meta.len(),
        meta.len() as f64 / 1_048_576.0
    );

    Ok(())
}
