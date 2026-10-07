/*
By: Aum markandey
Date: 2026-10-07
Program Details: Guessing game between 1-100
*/
    use std::io;
    use rand::Rng;
fn main() {
      println!("This is a guessing game. The numbers are between 1 and 100. Type 'exit' to quit the game.");
      let mut rng = rand::rng();
      let mut dice: i32 = rng.random_range(1..=100);
    loop{
     println!("Input your guess: ");  
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(parsed_value) = input.trim().parse::<i64>(){
    
    
    let ansFreeze = "FREEZING ";
    let ansCold = "Cold";
    let ansCool = "Cool";
    let ansWarm = "Warm";
    let ansHot = "Hot";
    let ansBoiling = "BOILING HOT";
    let ansCorrect = "Correct!";
    
    if parsed_value == dice as i64 {
        println!("{}", ansCorrect);
        dice = rng.random_range(1..=100);
        println!("Game has restarted. Guess the new number between 1 and 100.");
    } else if (parsed_value - dice as i64).abs() <= 5 {
        println!("{}", ansBoiling);
    } else if (parsed_value - dice as i64).abs() <= 10 {
        println!("{}", ansHot);
    } else if (parsed_value - dice as i64).abs() <= 20 {
        println!("{}", ansWarm);
    } else if (parsed_value - dice as i64).abs() <= 30 {
        println!("{}", ansCool);
    } else if (parsed_value - dice as i64).abs() <= 40 {
        println!("{}", ansCold);
    } else {
        println!("{}", ansFreeze);
    }
}
   else if input.trim().to_lowercase() == "debug" {
        println!("The correct number is: {}", dice);
    } 
    else if input.trim().to_lowercase() == "exit" {
        break;
    }else{
        println!("Invalid input. Please enter a whole number between 1 and 100 or type 'exit' to quit."); 
    }
    
    
   
    }
    
}
