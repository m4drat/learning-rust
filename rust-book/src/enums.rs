#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> String {
        String::from("Called!")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn plus_two(x: Option<i32>) -> Result<i32, std::error::Error> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let msg1: Message = Message::Move{x: 1, y: 2};
    let msg2: Message = Message::Quit;
    let msg3: Message = Message::Write("123".to_string());
    let msg4: Message = Message::ChangeColor(0, 1, 2);

    println!("msg1: {:?} -> {}", msg1, msg1.call());
    println!("msg2: {:?} -> {}", msg2, msg2.call());
    println!("msg3: {:?} -> {}", msg3, msg3.call());
    println!("msg4: {:?} -> {}", msg4, msg4.call());

    // Option examples
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);

    if let Some(number) = six {
        println!("Number six should be printed: {}", number);
    }

    if let Some(number) = none {
        println!("Number: {}", number);
    } else {
        println!("Got none!");
    }

}
