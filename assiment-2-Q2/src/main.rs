//Q1
// use std::io;
// fn main() {
//     loop {
//     let mut c = String::new();
//     println!("pls enter 1 number");
//     io::stdin().read_line(&mut c);
//     let c:f64=c.trim().parse().unwrap();

//     let mut a = String::new();
//     println!("add sing");
//     io::stdin().read_line(&mut a).expect("pls add sing");
//     let a:char=a.trim().parse().unwrap();

//     let mut d = String::new();
//     println!("pls enter 2 number");
//     io::stdin().read_line(&mut d);
//     let d:f64=d.trim().parse().unwrap();

//     if a == '+' {
//         println!("{}+{} = {}",d,c,d+c);
//     }else if a == '*'{
//         println!("{}*{} = {}",d,c,d*c);
//     }else if a == '/'{
//         println!("{}/{} = {}",d,c,d/c);
//     }else if a == '^'{
//         power((c as u64),(d as u64))
//     }else if a == '-'{
//         println!("{}-{} = {}",d,c,d-c);
//     }

//     if (c as u64) == 0 {
//         break
//         println!("Bey");
//     }

//     if d as u64 == 0 {
//         break
//         println!("Bey");
//     }
// }
// }
// fn power(x: u64, y: u64) {
//     let mut res: u64 = 0;

//     for _i in 1..y {
//         if res != 0 {
//             res = res * x ;
//         } else {
//             res = (x as u64) * (x as u64) ;
           
//         }
//     }
//    println!("{}^{} = {}",x,y,res);
// }

// //Q3

// fn main() {
//     let add_2 = |t:u32| -> u32 {
//         t+2
//     };
//     let mut r = Cacher{
//         closure:add_2,
//         value:None
//     };
//     r.value(6);
//     println!("{}",(r.closure)(5));
// }
// #[derive(Debug)]
// struct Cacher<T:Fn(u32) -> u32>{
//     closure: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> u32 {
//         calculation(6)
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.closure)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }

//Q4

// fn main() {
//     let child_1 = children {
//         Name:String::from("Aslam"),
//         education:String::from("bilingual"),
//     };
//     let child_2 = children {
//         Name:String::from("Asad"),
//         education:String::from("bilingual"),
//     };
//     println!("{} {}",child_2.Name(),child_2.education());
//     println!("{} {}",child_1.Name(),child_1.education());
//     adopt(child_1,child_2);
// }
// struct children {
//     Name:String,
//     education:String
// }
// trait quality {
//     fn education (&self) -> String;
//     fn Name(&self) -> String;
// }
// impl quality for children {
//     fn education(&self) -> String {
//         format!("the education of the children is {}",self.education)
//     }
//     fn Name(&self) -> String{
//         format!("{}",self.Name)
//     }
// }
// fn adopt<T:quality> (x:T,y:T) {
//     println!("I want to take {} and {} both children at y home",x.Name(),y.Name())
// }

