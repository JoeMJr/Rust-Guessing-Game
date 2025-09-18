use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    /* 
    println!("Hello, world!");
    println!("Guess a Number");
    println!("Please enter a number between 0 and 100");
    let mut guess = String::new();
    //let num = rand::rng().random_range(0..100);
    let num = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {num}");
    //println!("{}", num);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    //guess = guess.to_string();
    println!("You guessed: {}", guess.trim());
    
    //Put the random number thing here
    //let num = "50";
    let test = num.to_string();

    if guess.trim() == test {
        println!("You got it right, it was {}", num)
    }
    else{
        println!("You got it wrong, it was {}", num)
    }
    */
    // the tutorial I messed around with. I added a guess count var and lose condition
    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("Secret Num was {}.", secret_number);

    let mut guess_count = 5;

    loop {
        println!("Please input your guess. You have {} guesses", guess_count);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guess_count-=1;

        if guess_count == 0 {
            println!("You Lose! Ran out of guesses! The answer was {}.", secret_number);
            break;
        }
        else {
            println!("You have {} guesses left", guess_count);
        }
    }
}
