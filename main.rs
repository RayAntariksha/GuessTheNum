use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    //generating a random number that the user needs to guess
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut attempts = 0;
    loop {
        //taking the raw input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        //converting the raw input from a string into an integer 
        let guess : u32 = input.trim().parse().expect("Please type a number!");
        
        
        //Every time the loop starts over and the user enters an input,
        //we add 1 to the number of attempts
        attempts += 1;

        if guess == secret_number{
            println!("Congratulations! You guessed the number!");
            println!("Attempts: {}", attempts);
            // the loop breaks when user finds the correct number
            break;
        }else if guess < secret_number{
            println!("Too Low!");
        }else {
            println!("Too High!");
        }
        
    }

}