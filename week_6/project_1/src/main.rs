// Rust program for a Student council voting system

use std::io;


fn main() {
    // input ELigible candidates
    let mut input1 = String ::new();
    println!("\nCandidate number: {}",input1);
    io::stdin().read_line(&mut input1).expect("invalid string");
    let candidate_number:i32 = input1.trim().parse().expect("not a valid integer");
    

    //input name
    let mut input2 = String::new();
    println!("\nFull name :{}",input2);
    io::stdin().read_line(&mut input2).expect("invalid string");
    let _name = input2.trim();

    //input email
    let mut input3 = String::new();
    println!("\nEmail address: {}",input3);
    io::stdin().read_line(&mut input3).expect("invalid string");
    let _email_address = input3.trim();

    //input department
    let mut input4 =  String::new();
    println!("\nDepartment:{}",input4);
    io::stdin().read_line(&mut input4).expect("invalid string");
    let _department = input4.trim();

    //input state of origin
    let mut input5 = String::new();
    println!("\nState of origin: {}",input5);
    io::stdin().read_line(&mut input5).expect("invalid sting");
    let _state_of_origin = input5.trim();

    //input CGPA
    let mut input6 = String::new();
    println!("\nCGPA:{}",input6);
    io::stdin().read_line(&mut input6).expect("invalid string");
    let cgpa:f64 = input6.trim().parse().expect("invalid float");
    
    // input level
    let mut input7 = String::new();
    println!("\nLevel: {}",input7);
    io::stdin().read_line(&mut input7).expect("invalid string");
    let level:i32 = input7.trim().parse().expect("invalid integer");
    

    //input class position
    let mut input8= String::new();
    println!("\nAre you the class rep? {}",input8);
    io::stdin().read_line(&mut input8).expect("invalid string");
    let _class_position = input8.trim();

    if candidate_number < 150 && candidate_number > 150 && cgpa == 4.0 && _class_position == "yes" && level == 100
    {
        println!(" You can vote!");
    }
    else{
        println!("Sorry, you're not eligible to vote");
    }



}
