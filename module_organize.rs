use account::Account;
mod account{
  

pub struct Account{
  
  pub owner:String,
  pub balance:u32
}

impl Account{

pub fn new (owner:String,balance:u32)->Self{

    Self{
      
      owner:owner,
      balance:balance
    }
  }
  
  pub fn deposit(&mut self,amount:u32){
    println!("Account updated successfully");
    self.balance+=amount;
  }
  
  pub fn withdraw(&mut self, amount:u32){
    println!("Withdraw successfully");
    self.balance-=amount;
  }
  
}


}




fn main(){
  
  let mut acc =Account::new(String::from("Adil"),10_000);
  acc.deposit(100);
  acc.withdraw(50);
  println!("Owner of the account is : {}",acc.owner);
}


// mod math{
  
  
// pub fn add(a:u32, b:u32){
  
//   println!("New value is : {}",a+b);
// }

// }

// fn main (){
  
// math::add(10,12);
  
  
  
  
// }
