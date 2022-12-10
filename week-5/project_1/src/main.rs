  use std::io;

//a= coefficient of x^2, b = coffeficient of x and c = constant

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c:  ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let h:f32 = (b * b) - (4.0 * a * c);
    let t = h.sqrt();

    let o:f32 = -b + t;
    let p:f32 = -b - t;

    let x1:f32 = o / (2.0 * a);
    let x2:f32 = p / (2.0 * a);


    println!("The roots are {} and {}", x1, x2);
}
