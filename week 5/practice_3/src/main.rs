// Rust program to read the height of a person
// and then print if the person is tall , short 
// or average height

use std::io;


fn main() {
   
   let mut input = String//::new();


    println!("\n How Tall are you? (in centimeters):");
    io::stdin().read_line(&mut input).expect("This string is invalid");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 154.0 && height <= 170.0
    {
        println!("Omo!, you're average height sha");
    }
    else if height > 170.0 
    {
        println!("Haaa!!!, be like say nah you chop all the beans for your house");
    } 
    else if height < 150.0 
    {
        println!("I'm sorry to be the one to tell you but you're quite short");

    }
}
