// Rust program for NRG publication incentive system

use std::io; 

fn main() {
    // input researcher number
    let mut input1 = String::new();
    println!("\nNRG researcher number:{}",input1);
    io::stdin().read_line(&mut input1).expect("invalid string");
    let _researcher_number:i32 = input1.trim().parse().expect("invalid integer");
    

// input name
let mut input2 = String::new();
println!("\nName: {}",input2);
io::stdin().read_line(&mut input2).expect("invalid string");
let _name = input2.trim();



    // input  number of published papers
    let mut input3 = String::new();
    println!("\nHow many papers have you published? {} ",input3);
    io::stdin().read_line(&mut input3).expect("invalid string");
    let published_papers:i32 = input3.trim().parse().expect("invalid integer");


if published_papers >= 3 && published_papers <= 5 
{
    println!("Your incentive is N500_000");
}
if published_papers > 5 && published_papers < 10
{
    println!("Your incentive is N800_000");
}
if published_papers >= 10 
{
    println!("Your incentive is N1_000_000");
}
else if published_papers < 3 
{
    println!("Your incentive is N100_000");
}
}
