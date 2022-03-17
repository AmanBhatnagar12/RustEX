use std::io;
fn main(){ // Main method
    // First  Number
 println!("Enter the first number");

 let mut a=String::new();
 io::stdin().read_line(&mut a).expect("Not a Number");
// Enter Second Number 

 println!("Enter the second number=");
 let mut b=String::new();
 /*This actually reads */
io::stdin().read_line(&mut b).expect("Not a No ");
// Parsing into integer form 
let c: i32 = a.trim().parse().ok().expect("Program only processes numbers, Enter number");
let d: i32 = b.trim().parse().ok().expect("Program only processes numbers, Enter number");
println!("Sum ={}",c+d);
}