use std::collections::HashMap;

fn main() {
    let mut glossary: HashMap<String, String> = HashMap::new();
    
    glossary.insert(String::from("Ownership"), String::from("Rule of how Rust manages memory"));
    glossary.insert(String::from("Borrowing"), String::from("Referencing data without taking"));

    let search_term = String::from("hoge");
    let definition_to_add = String::from("Accessing data without taking ownership");

    glossary.insert(search_term, definition_to_add);

    let target_key = "hoge";
    
    //今回は学習のため、expectはつかわない
    let definition_result = glossary.get("hoge");

    match definition_result {
        Some(def) => {
            println!("\n--- Search Result ---");
            println!("Term: {}", target_key);
            println!("Definition: {}", def);
        }
        None => {
            println!("\nThe term '{}' is not found in the glossary.", target_key);
        }
    }
}