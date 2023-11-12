// Rust program to let customers order food from a resturant

use std::io;

fn main(){

     let mut cost = 0;

     println!("this is Jackie's resturant");
     println!("what would you like to have today?");
     println!("Menu\t  price(input to order)");
     println!("pounded yam & edinkaiko soup\t  N3_500(p)");
     println!("fried rice & chicken\t  N3_000(f)");
     println!("amala & ewedu soup\t  N2_500(a)");
     println!("eba & egusi soup\t  N2_000(e)");
     println!("white rice & stew\t  N2_500(w)");
     println!("input c to calculate order or to get bills for the order placed");
     println!("Note: To order a certain item twice input it twice in the system 
               So to say you input the letter for that meal once again after clicking enter");

     loop{
          let mut meal = String::new();
     io::stdin().read_line(&mut meal).expect("we do not have that here");
     let meal = meal.trim();

     if meal == "p"{
          cost+=3_500;
     }else if meal== "f"{
          cost+=3_000;
     }else if meal == "a"{
          cost+=2500;
     }else if meal == "e"{
          cost+=2_000;
     }else if meal == "w"{
          cost+=2_500;
     }else if meal == "c"{
          break;
     }else {
          println!("we do not serve that here");
          continue;
     }
     }

     if cost > 10_000{
          let discount_cost = cost - ((5 / 100) * cost);
          println!("Congrats!!, you've earned youself a discount from jackie's resturant");
          println!("New bill is {}", discount_cost );
     }else {
          println!("Your bill is {}", cost);
     }
println!("Thank you  for coming!!");
}