// fn main() {
//     let x = || println!("hello world");
//     x();
// }
// fn main() {
//     let x = |x:u32| x+1;
//     let y = 3;
//     println!("The function returns: {}",x(y)); 
// }
// fn main() {
//     let mut c = 1;
//     let mut x = || {c = c+1;};
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }
// Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
// passing a closure as an argument to a function
// fn print_message<T:Fn()>(x:T){
//     x();
//  }
//  fn main() {
//      let mut Print = || println!("This closure accepts no argument and returns nothing.It only print the message");
//      print_message(Print);
//  }
// // Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
// fn hello<T:Fn(u32)->u32>(x:T)->u32{
//     x(5)
// }
// fn main() {
//     let mut add_1 = |y| y + 1;
//     println!("{}",hello(add_1));
// }
