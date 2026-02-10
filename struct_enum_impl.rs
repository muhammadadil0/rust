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
