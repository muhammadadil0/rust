struct BankAccount{
  owner:String,
  balance:u32,
}

impl BankAccount{
  
   fn show_balance(&self){
     
     println!("Your Balance is : {}", self.balance);
   }  
   
   fn deposit(&mut self, amount:u32){
     self.balance += amount;
     
     println!("Amount is deposit successfully");
     
     println!("New Balance is : {}", self.balance);
     
   }
  fn withdraw(&mut self, amount:u32){
    
    if amount<=self.balance {
      
      self.balance-= amount;
      println!("Amount is withdraw successfully");
      println!("New Balance is :{} ",self.balance);
    }
    
  }
}


fn main(){
  
  let mut  user = BankAccount{
    owner:String::from("Adil"),
    balance:1_000_00,
  };
  
  user.show_balance();
  user.deposit(10);
  user.withdraw(10);
  
  
}
