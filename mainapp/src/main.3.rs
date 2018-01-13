
mod appmodule;
use std::vec;

#[derive(Debug)]
struct User {
    name: &'static str,
    age:i32
}


impl User {
  
  pub fn app(&self) {

      println!("this is a demo");
  }
  fn new_user() -> User {
    User{name:"dalong demo",age:333}
  }
}
struct AppDemo2<'a>{
    name: &'a str,
    power:&'a i32
}
impl <'a>  AppDemo2<'a> {
    
    pub fn demoapp(&self) -> &'a str{
      self.name
    }
    pub fn datavalidate(&self) ->Result<&str,i32>{

       if self.name.len()>20 {
           Err(22)
       }
       else{
           Ok(self.name)
       }

    }

    pub fn datavalidate2(&self) ->Result<i32,i32>{

       if *self.power>20 {
           Err(22)
       }
       else{
           Ok(*self.power)
       }

    }
}
  
fn mainww() {
   // result test demo 
   let appinfo =AppDemo2{name:"ddddddddddddddd",power:&12};
   let result= appinfo.datavalidate2();
   let myresult = match result {
       Ok(n) => {
            
            print!("the result ok is :{:?}",n*3)
       },
       Err(e) => println!("the error is:{}",e)
   };
   let newuser = User::new_user();

   println!("\r\n{:?}",newuser);
 }
