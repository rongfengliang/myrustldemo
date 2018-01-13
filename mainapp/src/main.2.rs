use std::vec;


struct AppDemo{
    name: &'static str,
    power: i32
}

struct AppDemo2<'a>{
    name: &'a str,
    power:&'a i32
}
 
 impl <'a>  AppDemo2<'a> {
    
    pub fn demoapp(self) -> &'a str{
      self.name
    }
 }
 


fn build_vector() ->Vec<i16> {

  let mut v= Vec::new();
  v.push(10);
  v.push(20);
  return v;

}

fn maine() {
 
   
//    let n =42;

//    match n {
//        ref r=>println!("got the result,{}",r),
//    }

//    let  mut m = 44;

//    match m {

//      ref mut mr =>{

//          println!("got info:{}",mr);
//          *mr=555;
//      },

//    }
//    println!("m has changed to:{}",m);

     let   app2 = AppDemo{name:"ddd",power:333}; 
     let  mut app = AppDemo2{name:"ddd",power:&333i32};
     println!("the info2 name:{},age:{}",app2.name,app2.power);
     let demo2=&app;
     let dmeo3=app;
     println!("call api result:{:?}",demo2.demoapp());
     println!("call api result:{:?}",dmeo3.demoapp());

     let info:Vec<i16> =build_vector();
     
     for item in info {
        println!("result info:{}",item);
     }


    let mut v= Vec::new();

    for i in 101..105 {

        v.push(i.to_string());
    }

    
  

   let str ="dalong demo app".to_string();

   let str2=str;
  
  let num :i32 =36;

  let num2=num;




 }
