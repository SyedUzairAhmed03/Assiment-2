1:

closures are functions which we can save in a variable
Example

let x = || println!("example of closure save in variable")

2:

closure can pass as argument in other function.
Example

fn other <T:Fn()> (x:T) {}

3: 

We can create the closure in one place and then call the closure to evaluate it in a different context
Example

fn main () {
let x = |num| num+1;
x(3)
}

4:

closures can capture values from the scope in which they are defined.

Example

fn main () {
let mut x = 1;
let y = || x = x+1;
}
