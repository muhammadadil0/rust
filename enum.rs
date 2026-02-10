enum Direction{
  Up,
  Down,
  Left,
  Right,
  
}

enum Message{
  Text(String),
  Number(i32)
  
}

enum TrafficLight{
  Red,
  Green,
  Blue
  
}



fn main(){

let light = TrafficLight::Red;

match light{
  TrafficLight::Red => println!("Red"),
  TrafficLight::Green => println!("Green"),
  TrafficLight::Blue=>println!("Blue"),
  
}

  
  let move1 = Direction::Up;
  
  let msg = Message::Text(String::from("hi my name is Muhammad"));
  
  
  match msg {
    Message::Text(m)=> println!("Text is : {}",m),
    Message::Number(n) => println!("Number is : {}",n)
  }
  
  
  match move1{
    
    Direction::Up => println!("Move Up"),
    Direction::Down => println!("Move Down"),
    Direction::Left => println!("Move Left"),
    _=> println!("Move Right"),
  }
  
}
