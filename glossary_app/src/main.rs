use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn  load_glossary(path : &str) -> Result<HashMap<String, String>, io::Error> {
    let mut map = HashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let content = line?;
        let parts: Vec<&str> = (&content).split(':').collect();

        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    Ok(map)
}

fn main() {
    let path = "glossary.txt";
    let glossary = load_glossary(path).expect("Failed to load glossary file");

    println!("---- Rust Glossary ----");
    let search_term = "Ownership";

    match glossary.get(search_term) {
        Some(def) => println!("Term: {}\nDefinition: {}", search_term, def),
        None => println!("Term '{}' not found in glossary.", search_term),
    }
}