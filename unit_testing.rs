fn multiply(a:u32,b:u32)->u32{
    a*b
}

fn subtract(x:u32,y:u32)->u32
{

     x -y

}
#[cfg(test)]
mod test{
//  use super::*; use this one or use crate
use crate::multiply;
use crate::subtract;
    #[test]
    fn test_multiply(){
        
        assert_eq!(multiply(4,5),20);
    }

    #[test]
    fn test_subtract(){

        let result = subtract(10,4);

        assert!(result>4);
        assert_eq!(result,6);
        assert_ne!(result,5);
    }
    
}
