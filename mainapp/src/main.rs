
extern crate rong;

use rong::Config;

#[derive(Debug)]
struct User {
    name: &'static str,
    age:i32
}
impl User {

    fn suminfo(&self){
       println!("this is call from struct:{}",self.name);
    }
}
pub trait Suminfo {
   
   fn app_demo(&self);
   fn summary(&self) {
     self.app_demo();
   }
}

impl Suminfo  for User {

   fn app_demo(&self) {
       println!("this is a demo:{}",self.name);
   }
}


fn longtest<'a>(x: &'a str,y: &'a str) ->&'a str {

    if x.len()>y.len() {
        x
    }
    else{
        y
    }
}
  
fn main() {
  
   let x= 4;

   let eq = |z| z==x;

   let y= 4;

   assert!(eq(y),"");

 }
