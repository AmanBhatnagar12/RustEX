use std::io;
fn main() {
    println!("Enter the value of A=");
    let mut a =String::new();
    io::stdin().read_line(&mut a).expect("{");
println!("Enter the value of B=");

    let mut b=String::new();
    io::stdin().read_line(&mut b).expect("{");

// Parsing 
let c:i32=a.trim().parse().ok().expect("Number is incorrect");
let d:i32=b.trim().parse().ok().expect("Number is incorrect");


    let sum = return_sum(c,d);
  println!("The sum of a &  b is = {}", sum);
  }
   
  fn return_sum(i: i32, j: i32) -> i32 {
   i+j
    
  }
 