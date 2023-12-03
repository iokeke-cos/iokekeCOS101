// rust program for user input of patient information
use std::io;

fn main() {
    println!(" Patient information :");

    //input name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalid string");
    println!("Full name : {}", name);

    //input date of birth
    let mut date_of_birth = String::new();
    io::stdin().read_line(&mut date_of_birth).expect("invalid input");
    let date_of_birth:i32 = date_of_birth.trim().parse().expect("Incorrect integer input");
     println!("Date of birth: {}",date_of_birth);

     //input age
     let mut age = String::new();
     io::stdin().read_line(&mut age).expect("invalid string");
     let age:i32 = age.trim().parse().expect("invalid integer");
     println!("Age: {}",age);

    // input email address
    let mut email_address = String::new();
    io::stdin().read_line(&mut email_address).expect("invalid string");
    println!("Email address: {}",email_address);

    //input phone number
    let mut phone_number = String::new();
    io::stdin().read_line(&mut phone_number).expect("invalid string");
    let phone_number:i32 = phone_number.trim().parse().expect("incorrect integer");
    println!("Phone number: {}",phone_number);

    //input number of sibling
    let mut number_of_siblings = String::new();
    io::stdin().read_line(&mut number_of_siblings).expect("invalid string");
    let number_of_siblings:i32 = number_of_siblings.trim().parse().expect("incorrect integer");
    println!("Number of siblings: {}",number_of_siblings);

    //input number of children
    let mut number_of_children = String::new();
    io::stdin().read_line(&mut number_of_children).expect("invalid string");
    let number_of_children:i32 = number_of_children.trim().parse().expect("incorrect integer");
    println!("Number of children: {}",number_of_children);

    //input medical diagnosis
    let mut medical_diagnosis = String::new();
    io::stdin().read_line(&mut medical_diagnosis).expect("invalid string");
    println!("Medical diagnosis:{}",medical_diagnosis);

    //input village of residence
    let mut village_of_residence = String::new();
    io::stdin().read_line(&mut village_of_residence).expect("invalid string");
    println!("Village of residence: {}",village_of_residence);

    if age > 50 && number_of_children == 4 && medical_diagnosis == "alzhemier"
    {
        println!("You get a 20% discount from the original bill which is N1_200_000");
    }
    else if age ==30 && number_of_siblings > 4 && village_of_residence == "Ngbauji"
    {
        println!("You have a discount of 5% of the original bill which is N550_000");
    }
    if medical_diagnosis == "chronic kidney disease" && age > 40 && number_of_siblings > 3 && number_of_children > 3 && village_of_residence == "Atabrikang"
    {
        println!("You have a discount of 15% of the originalbill which is 1_500_000 ");
    }
    if medical_diagnosis == "diabetes" && age > 28 && age < 45 && number_of_children >=2 && number_of_children <= 4
    && village_of_residence == "Okorobilom"
    {
     println!("You have a discount of 10% from the original bill which is N800_000 ");
    }
    if medical_diagnosis == "arthritis" && age > 58 && number_of_siblings > 5 && number_of_children > 5 && village_of_residence == "Emeremen"
    {
        println!("You have a discount of 10% of the original bill which is N450_000");
    }



}
