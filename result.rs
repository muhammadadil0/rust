struct Student{
  name:String,
  grade:Option<i32>
}


fn get_grade(student:&String,student_db:&Vec<Student>)->Option<i32>{
  
  for stud in student_db{
    
    if stud.name == *student {
      
      return stud.grade;
      
    }
    
  }
  None
  
}


fn check_student(student:&String, student_db:&Vec<Student>)->Result<(),String>{
  
  for stud in student_db{
    
    if stud.name == *student{
      
      return Ok(())
    }
  }
  Err(String::from("Student Not Found"))
  
}



fn main(){
let student_db = vec![
  Student{
    name:String::from("Adil"),
    grade:Some(90)
    
  },
  
  ];
  
  
  let name : String = String::from("Adil");
  
  let student_found = check_student(&name,&student_db);
  
  
  match student_found{
    
    Ok(_)=> {
      
      let student_grade = get_grade(&name,&student_db);
      
      if let Some(g) = student_grade{
        println!("Grade is : {}",g);
      } 
    }
    Err(msg)=>println!("{}",msg)
  }
  
  // let grade = get_grade(&name,&student_db);
  
  // match grade {
    
  //   Some(n) => println!("Grade is : {}", n),
    
  //   None => println!("Error no grade found")
  // }
  
  
 
  
}
