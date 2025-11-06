fn append_exclamation(s: &mut String){
    s.push_str("!");
}

fn main(){
    let mut message = String::from("Complete");

    append_exclamation(&mut message);

    println!("Changed message: {}", message);
}
