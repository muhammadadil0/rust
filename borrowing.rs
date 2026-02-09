// fn change(s:&mut String){
//   s.push_str(" bhai");
//   println!("name is : {}",s);
// }



fn main(){
  

  
  
  let mut value :i32 = 30;
  
  let ref1 = & mut value;
  
  let deref = *ref1;
  *ref1 = 10;
  println!("deref value is : {}",deref);
  println!("ref value is : {}",ref1);
  
  let mut vec : Vec<i32> = vec![12,44,4];
  
  let ref1 = &mut vec;
  
  ref1.push(8);
  
  (*ref1).push(8);
  
  println!("vec is : {:?}",vec);
  
  
  // change(&mut name);
  // println!("name is : {}",name);
 
 
 
 
  
}
