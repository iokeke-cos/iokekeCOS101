// Rust program to determine age pass

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string, try again -_-");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("This string is invalid, sir!!");
    let age:i32 = input2.trim().parse().expect(" i am sorry but your number is invalid ");

    if age >= 16  {
        println!("Welcome to the best party of all time {}!!",input1);
    }  
         else{
            println!(" o boy go back you are too young for this party Just see how young you are {}",input2);
         }
}
