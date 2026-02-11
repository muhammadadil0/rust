enum AccountStatus{
  Active,
  Blocked
  
}

struct User{
  
  name:String,
  status:AccountStatus
}

impl User{
  
  fn show_status(&self){
    
    match self.status{
      AccountStatus::Active => println!("User is active"),
      AccountStatus::Blocked =>println!("User is blocked")
    }
    
  }
  
  fn block(&mut self){
    self.status = AccountStatus::Blocked;
  }
}

fn main(){
  
  let user = User{
    name:String::from("Adil"),
    status:AccountStatus::Active
  };
  
  user.show_status();
}


____________________________________________________- Struct _____________________________________________________




struct Car{
  
  owner:String,
  year:u32,
  price:u32,
}

impl Car{
  
  fn display(&self){
    println!("Owner is : {} Year is : {} Price is : {}",self.owner, self.year, self.price);
  }
  
  
  fn change_Price()->u32{
    40_000
    
  }
  
  fn new_price (&self)->u32{
    self.price + Self::change_Price()
  }

  
   fn insert(owner:String,year:u32,price:u32)->Self {
    
    Self{
      owner:owner,
      year:year,
      price:price
    }
  }
}


 


fn main(){
  
  let my_car = Car {
    owner:String::from("Adil"),
    year:2021,
    price:30_000
    
  };
  
  
  my_car.display();
  let new_price = my_car.new_price();
  println!("New price is : {}",new_price);
  
  let new_owner = Car::insert(String::from("Adnan"),2023,30_000);
  
  
  println!("New owner is : {}", new_owner.owner);
  
}

