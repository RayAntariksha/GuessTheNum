use rand::Rng;
use std::io;

fn main() {
    // opening text
    println!("Guess the number!");
    println!("hello");
    println!("Please enter the difficulty level: (1)Easy, (2)Medium, (3)Hard");

    //difficulty level

    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");

    let difficultyint: i32 = difficulty.trim().parse().expect("Please type a number!");

    let mut difficulty_level: i32 = 0;

    if difficultyint == 1 {
        difficulty_level = 10;
    } else if difficultyint == 2 {
        difficulty_level = 8;
    } else {
        difficulty_level = 5;
    }

    //generating a random number that the user needs to guess
    let secret_number = rand::rng().random_range(1..101);

    let mut attempts = 0;
    loop {
        //taking the raw input from the user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //converting the raw input from a string into an integer
        let guess: u32 = input.trim().parse().expect("Please type a number!");

        //Every time the loop starts over and the user enters an input,
        //we add 1 to the number of attempts
        attempts += 1;

        if guess == secret_number {
            println!("Congratulations! You guessed the number!");
            println!("Attempts: {}", attempts);
            // the loop breaks when the player finds the correct number
            break;
        } else if guess < secret_number {
            println!("Too Low!");
        } else {
            println!("Too High!");
        }
        if attempts == difficulty_level {
            println!("GAME OVER! The number was: {}", secret_number);
            break;
        }
    }
}
