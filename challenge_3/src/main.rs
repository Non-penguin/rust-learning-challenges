fn get_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main(){
    let string1 = String::from("Longer stiring");
    let result;

    {
        let string2 = String::from("Short");
        result = get_longer(string1.as_str(), string2.as_str());
        println!("The longer string is: {}", result);
    }

    println!("The longer string is: {}", string1);
}
