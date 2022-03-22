use std::io;
fn main() {
    println!("Enter a number");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Not a number");
    let num_a:i32=num.trim().parse().ok().expect("Hububu");
    if num_a%2==0{
        println!("Number is even = :)");
    }
    else {
        println!("Number is odd = :)");
    }
}
