use std::io::Read;


fn main() {
    let mut file = std::fs::File::open("Lager_drinks.txt").unwrap();
    let mut l_drinks = String::new();
    //l_drinks stands for lager drinks eg 33 Export, etc.
    file.read_to_string(&mut l_drinks).unwrap();
    println!("{}",l_drinks);

    let mut file = std::Fs::file::open("Stout_drinks.txt").unwrap();
    let mut stouts = String::new();
    //stouts stands for stout drinks eg legend ,etc.
    file.read_to_string(&mut stouts).unwrap();
    println!("\n{}", stouts);

    let mut file = std::fs::File::open("Non-alcoholic_drinks.txt").unwrap();
    let mut no_alcohol = String::new(); 
    // no_alcohos stands for non_alcoholic_drinks eg Fayrouz,etc.
    file.read_to_string(&mut no_alcohol).unwrap();
    println!("\n{}",no_alcohol);

}

