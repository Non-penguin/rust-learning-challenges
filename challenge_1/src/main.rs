fn consumes_string(s: String){
    println!("Consumed string: {}", s);
}

fn borrows_string(s: &String){
    println!("Borrowed string: {}", s);
}

fn main(){
    let s1 = String::from("所有権が移動するデータ");
    let s2 = String::from("参照として貸し出すデータ");

    consumes_string(s1);
    borrows_string(&s2);

    println!("s2 is still vailable: {}", s2);
}
