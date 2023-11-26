use std::io;

// function for trapezium.
fn trapezium() {
    
     println!("\nEnter height");
     let mut input1 = String::new();
     io::stdin().read_line(&mut input1).expect("not a valid string");
     let input1:f32 = input1.trim().parse().expect("incorrect float");

     println!("\nEnter base1 :");
     let mut input2 = String::new();
     io::stdin().read_line(&mut input2).expect("not a valid string");
     let input2:f32 = input2.trim().parse().expect("incorrect float");

     println!("\nEnter base2");
     let mut input3 = String::new();
     io::stdin().read_line(&mut input3).expect("not a valid string");
     let input3:f32 = input3.trim().parse().expect("incorrect float");

     let mut area:f32 = input1 / 2 * (input2 + input3);

     println!("Area of trapezium is {}",area;

}

fn rhombus(){

    println!("\nEnter value for  first diagonal:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect(" not a valid string");
    let input4:f32 = input4.trim().parse().expect("incorrect float");

    println!("\nEnter value for second diagonal: ");
    

}
