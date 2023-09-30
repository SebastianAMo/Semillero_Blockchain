// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor((u8, u8, u8)),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo(String::from("Hello")));
        println!("{:?}", Message::Move { x: 1, y: 2 });
        println!("{:?}", Message::ChangeColor((255, 0, 255)));    
    }
}

