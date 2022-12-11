use std::io;

fn main() {
    // Ask the number of siblings
    println!("How many siblings do you have?");
    let mut num_siblings = String::new();
    io::stdin().read_line(&mut num_siblings).expect("Failed to read input");
    let num_siblings:u32 = num_siblings.trim().parse().expect("Please enter a valid number");

    // Create a vector to store the data for each sibling
    let mut siblings:Vec<String> = Vec::new();

    // Loop through each sibling and collect their data
    for i in 0..num_siblings {
        println!("Enter the first name of sibling #{}:", i + 1);
        let mut first_name = String::new();
        io::stdin().read_line(&mut first_name).expect("Failed to read input");

        println!("Enter the age of sibling #{}:", i + 1);
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read input");
        let age:f64 = age_str.trim().parse().expect("Please enter a valid number");

        let mut data = Vec::new();
        data.push(first_name);

        

        if age > 18.0 {
            println!("Is sibling #{} married or single?", i + 1);
            let mut marital_status = String::new();
            io::stdin().read_line(&mut marital_status).expect("Failed to read input");
            marital_status = marital_status.trim().to_lowercase();
            

            if marital_status == "single" {
                println!("Is sibling #{} a student or a worker?", i + 1);
                let mut student_or_worker = String::new();
                io::stdin().read_line(&mut student_or_worker).expect("Failed to read input");
                student_or_worker = student_or_worker.trim().to_lowercase();

                if student_or_worker == "student" {
                    println!("What university does sibling #{} attend?", i + 1);
                    let mut university = String::new();
                    io::stdin().read_line(&mut university).expect("Failed to read input");
                    println!("What is sibling #{}'s course of study?", i + 1);
                    let mut course = String::new();
                    io::stdin().read_line(&mut course).expect("Failed to read input");

                    data.push(marital_status);
                    data.push(student_or_worker);
                    data.push(university);
                    data.push(course);
                   
                } else if student_or_worker == "worker" {
                    data.push(marital_status);
                    data.push(student_or_worker);
                } else {
                    println!("Invalid input for sibling #{}'s occupation", i + 1);
}   }   }   }   }