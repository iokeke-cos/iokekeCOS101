fn main() {
    
    //a list of nos
    let v = vec![15, 25, 35 , 45 , 55];
    print_vector(v.clone());  
    // this line above  gives an error 
    
    //instead of print_vector(v) like in practice do as shown here
    
    println!("{}",v[0]);  

}

fn print_vector(x:Vec<i32>){

    println!("Inside print_vector function {:?}",x);

}
