enum Message{
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main(){
    let msg_move = Message::Move{ x: 10, y: 20};
    let msg_write = Message::Write(String::from("Hello, Rust!"));

    process_message(msg_move);
    process_message(msg_write);
}

fn process_message(msg: Message){
    match msg {
        Message::Quit => {
            println!("Received Quit message.");
        }

        Message::Move{ x, y } => {
            println!("Moving command: x={}, y={}", x, y);
        }

        Message::Write(text) => {
            println!("Writing message: {}", text);
        }

        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b);
        }
    }
}
