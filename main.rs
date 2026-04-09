use std::io;
use rand::prelude::*;
fn main(){
    //generate a rng for the random commands
    let mut rng = rand::rng();
    let target = rng.random_range(1..=100);
    println!("Guess the value from 1 - 100");
    
    let mut attempts = 1;
    loop{
        let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            let res : i16 = user_input.trim().parse().expect("Input not an integer");
        if res == target{
            break
        }
        else if res > 100 || res < 0{
            println!("Value out of range, has to be between 0 and 100");
        }
        else if res > target{
            println!("Lower");
        }
        else{
            println!("Higher");
        }
        attempts += 1;
    }
    print!("Congrats!, you got the value in {attempts} number of attempts");
}
