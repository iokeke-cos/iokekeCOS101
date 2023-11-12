use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("input the value of a");
    io::stdin().read_line(&mut input1).expect("not a valid figure");
    let a:f32 = input1.trim().parse().expect("not a valid figure");

    println!("input the value of b");
    io::stdin().read_line(&mut input2).expect("not a valid figure");
    let b:f32 = input2.trim().parse().expect("not a valid figure");

    println!("input the value of c");
    io::stdin().read_line(&mut input3).expect("not a valid figure");
    let c:f32 = input3.trim().parse().expect("not a valid figure");

    let discriminant:f32 = (b.powf(2.0)) - (4.0 * a * c);

    if discriminant > 0.0{
        println!("there are two distinct roots");
    }else if discriminant == 0.0{
        println!("there is only one distinct root");
    }else if discriminant < 0.0{
        println!("there is no real root");
    }else{
        println!("non existent");
    }
}