use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    println!("\nPlease Enter your name");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Change your input bruh");
    println!("your name is: {}", name); 

    println!("\nEnter your age");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Change your input bruh");
    let age:i32 = age.trim().parse().expect("Thats not an integer");
    println!("your age is: {}", age);
}
