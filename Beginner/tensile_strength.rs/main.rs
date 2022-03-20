
use std::io;
fn main(){
println!("Input Tensile strength =");
 
let mut var1 = String::new();

io::stdin().read_line(&mut var1).expect("Unable to read entered data");
let var2:i32=var1.trim().parse().ok().expect(":{");
if var2>=50&& var2<=90{
    println!("Very Good");
}
else {
println!("Not Good");
}
}