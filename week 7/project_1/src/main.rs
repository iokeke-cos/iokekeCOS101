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

     println!("Area of trapezium is {}",area);

}


// function for rhombus.
fn rhombus(){

    println!("\nEnter value for  first diagonal:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect(" not a valid string");
    let input4:f32 = input4.trim().parse().expect("incorrect float");

    println!("\nEnter value for second diagonal: ");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("not a valid string");
    let input5:f32 = input5.trim().parse().expect("incorrect float");

    let mut area:f32 = 1/2 * input4 * input5;

    println!("Area of rhombus is {}", area);

}


// function for parallelogram.
fn parallelogram(){

    println!("\nEnter value for base :");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("not a valid string");
    let input6:f32 = input6.trim().parse().expect("incorrect float");

    println!("\nEnter value for altitude :");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("not a valid string");
    let input7:f32 = input7.trim().parse().expect("not a valid float");

    let mut area:f32 = input6 * input7;

    println!("Area of parallelogram is {}",area);


}


// function for cube.
fn cube(){

    println!("\nEnter value for length of side:");
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("not avalid string");
    let input8:f32 = input8.trim().parse().expect("not a valid float");

    let mut area:f32 = 6 * input8.powf(2.0);

    println!("Area of cube is {}",area);
}

// function for volume of cylinder.
fn cylinder(){
   

    println!("\nEnter the value for radius:");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("not a valid string");
    let input9:f32 = input9.trim().parse().expect("incorrect float");

    println!("\nEnter value for height :");
    let mut input10 = String::new();
    io::stdin().read_line(&mut input10).expect("not a valid string");
    let input10:f32 = input10.trim().parse().expect("incorrect float");

    let mut volume:f32 = 3.24 * input9.powf(2.0) * input10;

}

fn main(){
    
}