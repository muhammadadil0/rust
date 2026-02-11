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










// enum TravelType{
//   Car,
//   Train,
//   Aeroplane,
   
// }

// impl TravelType{
  
//     fn travel_allownance(&self, miles:f32)->f32{
      
//   let allownance = match self{
//           TravelType::Car => miles * 0.2,
//           TravelType::Train => miles * 0.4,
//           TravelType::Aeroplane => miles * 0.8
//       };
//       allownance
//     }
// }

enum TravelType {
  Car(f32),
  Train(f32),
  Aeroplane(f32)
}

impl TravelType{
  
  fn travel_allownance(&self)->f32{
    
    let allownance = match self {
      
      TravelType::Car(miles) => miles * 0.1,
      TravelType::Train(miles) => miles * 0.2,
      TravelType::Aeroplane(miles) => miles * 0.3,
    };
    allownance
    
  }
}

fn main(){
  
  let participant = TravelType::Car(30.1);
  
  let result =  participant.travel_allownance();
  
  println!("Result is : {}", result);
   
}
