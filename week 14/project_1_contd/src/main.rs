use std::io::Read;
use::std::io;

fn main(){
    
    println!("WELCOME TO THE GLOBACOM LTD DATABASE!        Enter:\n(a) If you are an administrator \n(p) If you are a project manager 
(e) If you are an employee \n(c) If you are a customer \n(v) If you are a vendor    
");
   
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("incorrect string");
    let input1 = input1.trim();

    let input2= ["a", "p", "e", "c", "v"];
    let user= [admin_table, project_m_table, staff_table, custo_table, vend_table];

    for x in 0..5 {
     
     if input1 == input2[x]{

        user[x]();
        return;
     }

    }

    println!("incorrect input");
     

}

fn admin_table(){

    let mut file1 = std::fs::File::open("globalcom_dbase.sql").unwrap();
    let mut input3 = String::new();
    file1.read_to_string(&mut input3).unwrap();
    println!("{}", input3);

}

fn project_m_table(){

    let mut file2 = std::fs::File::open("project_tb.sql").unwrap();
    let mut input4= String::new();
    file2.read_to_string(&mut input4).unwrap();
    println!("{}", input4);
}

fn staff_table() {

    let mut file3 = std::fs::File::open("staff_tb.sql").unwrap();
    let mut input5 = String::new();
    file3.read_to_string(&mut input5).unwrap();
    println!("{}", input5);
}

fn custo_table() {
    
    let mut file4 = std::fs::File::open("customer_tb.sql").unwrap();
    let mut input6 = String::new();
    file4.read_to_string(&mut input6).unwrap();
    println!("{}", input6);
}

fn vend_table() {

    let mut file5 = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut input7 = String::new();
    file5.read_to_string(&mut input7).unwrap();
    println!("{}", input7);
}
