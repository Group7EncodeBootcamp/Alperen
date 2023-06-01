// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM DONE

#[derive(Debug)]
enum Message {
    Quit = 2,
    Echo = 5,
    Move = 7,
    ChangeColor = 8,
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
