fn main() {
    let n1 = "Electrical".to_string();
    let n2 = " Electronic".to_string();
    let n3 = " Engineering".to_string();
    let n4 = n1 + &n2 + &n3;

    println!("\nThe {} is informed by the aspiration to train electrical/electronic enginnering professionals in the areas of design, building and maintenance of electriccal control systems,",n4);

    let w1 = "computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2;

    println!("\n {} is aimed at developing competent, creative,
        innovative, entrepreneurial and ethically-minded persons,
        capable of creating value in the diverse fields of Computer Science.",w3);
}
