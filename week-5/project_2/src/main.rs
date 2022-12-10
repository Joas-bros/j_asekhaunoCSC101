use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("NOTE: If you are experienced input 1, if you are inexperienced input 2 ");
   
   println!("Enter your age:  ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let age:f32 = input1.trim().parse().expect("Not a valid number");


   println!("Enter your Experience No.:  ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let e:f32 = input1.trim().parse().expect("Not a valid number");

   if age >= 40.0 && e == 1.0{
    println!("Incentive is $1,560,000");
   }
    if age >= 30.0 && age < 40.0 && e == 1.0{
    println!("Incentive is $1,480,000");
   }
    if age < 28.0 && e == 1.0{
    println!("Incentive is $1,300,000");
   }
   else if e != 1.0 {
    println!("Incentive is $100,000");
   }
  
   
   






}
