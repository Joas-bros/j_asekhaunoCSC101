use std::io;
fn main() {
    println!("Hello traveller\nyou have encountered dabloons?\nWonderful,how many did you get?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
   let dabloons:f32 = input1.trim().parse().expect("Not a valid number");

   if dabloons > 1_000_000.0{
    println!("\nWow you have so many xd!!!");
   }

   if dabloons == 664166.0{
    println!("DEMON SPAWN!!!!!!");
   }

   let c:f32 = 2500.0 + dabloons;

   println!("\nYou have {} dabloons",c);

}