use std::io;

fn information(name: &mut Vec<String>, degree: &mut Vec<String>, experience: &mut Vec<usize>){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nFull name:");
    io::stdin().read_line(&mut input1).expect("invalid input");
    let your_name = input1.trim();
    name.push(your_name.to_string());

    println!("\nWhat course do you have a degree in?");
    io::stdin().read_line(&mut input2).expect("invalid");
    let bsc = input2.trim();
    degree.push(bsc.to_string());

    println!("\nHow experienced are you as a programmer?( ie your work experience)");
    io::stdin().read_line(&mut input3).expect("invalid");
    let years:usize = input3.trim().parse().expect("invalid");
    experience.push(years);
}

fn main(){
    let mut name: Vec<String> = Vec::new();
    let mut degree: Vec<String> = Vec::new();
    let mut experience: Vec<usize> = Vec::new();
   
    let mut input4 =String::new();
    println!("\nHow many persons are to be interviewed??");
    io::stdin().read_line(&mut input4).expect("invalid");
    let to_be_interviewed:usize = input4.trim().parse().expect("invalid");

    for j in 0..to_be_interviewed{
        information(&mut name, &mut degree, &mut experience);
    }

    let most_experience = checker(to_be_interviewed, &experience);
    println!("\nThe person with the most experience is {}. With a degree of {} and {} years of experience",name[most_experience], degree[most_experience],
        experience[most_experience]);
}

fn checker(input5:usize, years: &Vec<usize>) -> usize {
    let mut experience = 0;
    let mut max_experience = 0;
    for j in 0..input5{
        if years[j] > experience{
            experience = years[j];
            max_experience = j;
        }
    }
    return max_experience;
}
