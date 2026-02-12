

use std::collections::HashMap;


fn main(){
  
  let mut count_words : HashMap<&str,i32>  = HashMap::new();
  count_words.insert("Hello",2);
  count_words.insert("Programming",1);
  
  let new_entry = count_words.entry("C++").or_insert(0);
let has_key = count_words.contains_key("Hello");
 
let get_value = count_words.get("Hello");
 
 
  match get_value{
  Some(g)=>println!("Value is : {}",g),
  None => println!("Value is not found")
   
}
println!("Key is : {}",has_key);
  println!("count words is : {:?}",count_words);
}




























