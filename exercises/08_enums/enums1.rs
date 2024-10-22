#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize = 5,
    Move = 8,
    Echo = 0,
    ChangeColor = 52,
    Quit = 9,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
