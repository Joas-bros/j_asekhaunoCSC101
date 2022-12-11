use std::io;
// areaoftrapezium=T,areaofrhombus=R,areaofparallelogram=P,areaofcube=B,volumeofcylinder=C

fn main() {
   
    let mut equation = String::new();
    io::stdin().read_line(&mut equation).expect("Not a valid float");
    let equation_values:f32 = equation.trim().parse().expect("not a valid something");

    println!("\nArea of trapezium = 1
        \nArea of rhombus formula = 2 
        \nArea of parallelogram = 3
        \nArea of cube = 4
        \nArea of cylinder = 5");

    if equation_values == 1.0{
        area_of_trap();
    }
    if equation_values == 2.0{
        area_of_rhom();
    }
     if equation_values == 3.0{
        area_of_para();
    }
     if equation_values ==4.0{
        area_of_cube();
    }
     if equation_values == 5.0{
        area_of_cyli();
    }

}

fn area_of_trap() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of Height:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of Base1:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let base_1:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of Base2:  ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let base_2:f32 = input3.trim().parse().expect("Not a valid number");

    let o:f32 = height / 2.0;
    let p:f32 = base_1 + base_2;

    let j:f32 = o * p;
    println!("The area of the trapezium is {}",j);


3
}

fn area_of_rhom() {
    let mut input1 = String::new();
    let mut input2 = String::new();
   

    println!("Enter value of Diagonal 1:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let diag_1:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of Diagonal 2:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let diag_2:f32 = input2.trim().parse().expect("Not a valid number");

    let u:f32 = (diag_1 * diag_2) / 2.0;
    println!("The area of the rhombus is {}",u);
}

fn area_of_para() {
    let mut input1 = String::new();
    let mut input2 = String::new();
   

    println!("Enter value of base:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of altitude:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let alt:f32 = input2.trim().parse().expect("Not a valid number");

    let q:f32 = base * alt;
    println!("The area of the parallelogram is {}",q);
}

fn area_of_cube() {
    let mut input1 = String::new();
    
   

    println!("Enter value of length:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let len:f32 = input1.trim().parse().expect("Not a valid number");

  

    let w:f32 = (len * len) * 2.0;
    println!("The area of the rhombus is {}",w);
}

fn area_of_cyli() {
    let mut input1 = String::new();
    let mut input2 = String::new();
   

    println!("Enter value of Radius:  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let rad:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of height:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let hei:f32 = input2.trim().parse().expect("Not a valid number");

    let x:f32 = 22.0 / 7.0;

    let d:f32 = x * (rad * rad) / hei;
    println!("The area of the cylinder is {}",d);
}








