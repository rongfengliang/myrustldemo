 #[derive(Debug)]
pub struct Config {
   pub name: &'static str,
   pub age: i32
}
impl Config {
   pub fn new()->Config{
      Config{
          name:"dalongdemo",
          age:333
      }
   }
}
pub fn appdemo() -> &'static str{
    "dalong demo app"
}