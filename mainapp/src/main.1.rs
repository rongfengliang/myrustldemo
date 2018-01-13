
struct User {
 name :& 'static str,
 age: i32
}

struct AppDemo<T>{

    first:T,
    second:T
}

impl<T> AppDemo<T> {

    pub  fn printresult2(self){
        println!("the demo app");
    }
}


impl User {

 pub  fn printuserinfo(&self) -> &str{
     self.appdemo();
     return self.name;
 }
 fn  appdemo(&self){

     println!("this is a demo\n");
 }
}


enum Compass {

  North,South,East,West

}
#[derive(Debug)]
enum Compass2 {

   App(&'static str,i32),
   Mobile(&'static str,i32)

}
enum Result<T,E>{

    Ok(T),
    Err(E)
}

struct Player {

    name: &'static str,

    health:i32,

    leavel: u8
}

mod app;
pub fn  printresult(){

    println!("the result print is")
}
use std::vec;
fn maindemoapp() {

    let  number =100;
    let  demoapp = if  number>0 {true} else{false};
    // struct

    // let usertype = User{name:"dalong dmeo",age:444};
    // let result=  usertype.printuserinfo();
    // app::rongdemo();
    // println!("the result is :{}\n",result);
    // println!("user type name is :{}\n",usertype.printuserinfo());


    // data content

    // let number =100;
    // let a =&number;
    // printresult();
    // println!("the address,{:p}",&number);
    // println!("the address,{:p}",&a);
    // println!("the address,{}",*a);
   //println!("the content is :{:p}",*number);

    // generic

    // let appdemo  = AppDemo<i32>{first:333,second:67};
    // appdemo.printresult2();


    // string
    // let mut  mustinfo = String::new();

    // mustinfo.push('s');

    // mustinfo.push_str("this is a demo");
    // if &mustinfo == "s" {

    //     println!("is ok ");
    // }

    // for item in mustinfo.chars() {

    //     println!("the char is :{}",item);
    // }

    // println!("the finly stirng information is:{}",mustinfo)

    //vector

    // let  items= ["first","second"];

    // println!("the length:{}",items.len());
    // for  item in  items.iter()  {

    //      println!("the item info is:{}",item)

    // }

    // let mut  numbers =vec![23,66,77,4,555,111,3];
    // let  mut   items:Vec<i32> =(0..10).collect();
    // items.push(33);
    // items.push(33);
    // items.push(33);
    // items.push(33);
    // items.push(33);
    // let ids =&numbers[1..3];
    // for item in &items[1..14] {
    //  println!("the item is:{}",item);

    // }

   // tulp   just like lua next  decustructor
    //  let appdemo = ("dalong",333);

    // let (name,age) = ("dalong",333);

    // println!("name:{},age:{}",name,age);

    // println!("name:{},age:{}",appdemo.0,appdemo.1);

//    let mut  pl1= Player{name:"dddd",health:33,leavel:3};

//    pl1.leavel=4;


//    let    demoplayer = &Player{name:"dddd",health:33,leavel:3};

//    println!("name:{},health:{},leavel:{}",(*demoplayer).name,demoplayer.health,demoplayer.leavel);
//    println!("name:{},health:{},leavel:{}",pl1.name,pl1.health,pl1.leavel);


  let appname ="dalong";

  let demofn = |n:i32| -> &'static str {
      println!("inoput result is :{}",n);
     // let info ="dalongddd";
      return  "aaaa";
  };
  match appname {
          "dalong" => println!("all things dalong"),
          _ => println!("there is no value")
  };

  if appname.len()<1000 {

      panic!("this is some wrong with the length of app");
  }

 }
