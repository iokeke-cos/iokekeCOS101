// Rust program to calculate speed of a car going 80 miles in 2 hours.
fn main (){

    let  d : f64  = 80.0 / 0.6214;
    let  t : f64 = 2.0;
    let speed : f64 = d / t;
    println!("The speed of the formular one car is {}", speed);

    // second program for the same car but with different inputs for distance and time.
    let d : f64 = 120.0/ 0.6214;
    let t : f64 = 4.0;
    let speed : f64 = d / t;
    println!("The speed for the formular one car but with distance 120 miles and time 4 hours is {} ", speed);


}
