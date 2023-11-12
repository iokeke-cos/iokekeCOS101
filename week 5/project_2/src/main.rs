
// Rust program to determine the annual incentive of the employees in a company

use std::io;

fn main() {
   let mut input1 = String::new(); 
   let mut input2 = String::new();
   

   println!("\n Are you an experienced employee? (yes/no)");
   io::stdin().read_line(&mut input1).expect("Invalid string");
   let experienced = input1.trim();

   println!("\nEnter your age :");
   io::stdin().read_line(&mut input2).expect("Invalid string");
   let age:i32 = input2.trim().parse().expect("Invalid number");

 if experienced == "yes"{
    if age >= 40
   {
    println!("Your annual incentive is 1_560_000 naira.");
   }
    if age >= 30 && age < 40
   {
    println!("Your annual incentive is 1_480_000 naira.");
   }
   else  if age < 28
   {
    println!("Your annual incentive is 1_300_000 naira.");
   } 
 } 
  
if experienced == "no"{
    println!("Your annual incentive is 100_000 naira");
}

}
