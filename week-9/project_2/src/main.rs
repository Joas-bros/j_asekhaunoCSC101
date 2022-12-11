use std::fs::File;
use std::io::prelude::*;
use std::io::Result;


struct Student {
    name: String,
    matric: String,
    department: String,
    level: u64,
}

fn main() -> Result<()> {
    // Create an array or vector of students
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Bianca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100
        }
    ];

    
    for student in &students {
        println!("Name: {}", student.name);
        println!("Matric.Number: {}", student.matric);
        println!("Department: {}", student.department);
        println!("Level: {}", student.level);
        println!("---");
    }

    
    let mut file = File::create("students.txt")?;

    
    for student in &students {
        file.write_all(format!("{}\n", student.name).as_bytes())?;
        file.write_all(format!("{}\n", student.matric).as_bytes())?;
        file.write_all(format!("{}\n", student.department).as_bytes())?;
        file.write_all(format!("{}\n", student.level).as_bytes())?;
        file.write_all(b"---\n")?;
    }

    Ok(())
}
