struct Student{
  
  name:String,
  grade:Option<u32>
}




fn check_student (student:&String,student_db:&Vec<Student>)->Option<u32>{
  
  
  println!("Name is : {}",student);
  
  for stud in student_db{
    if stud.name == *student{
      return stud.grade
    }
  }
  None
}
fn main(){
  
  let student_db = vec![
    
    Student{
      
      name:String::from("Adil"),
      grade:Some(90)
    },
    Student{
      
      name:String::from("Azeem"),
      grade:Some(95)
    },
    Student{
      
      name:String::from("kaif"),
      grade:Some(90)
    },
    
    ];
    
    let student :String = String::from("Adil");
    let result = check_student(&student,&student_db);
    
    
    match result{
      Some(grade) => println!("Grade is : {}",grade),
      None => println!("Student not found")
    }
  
}
