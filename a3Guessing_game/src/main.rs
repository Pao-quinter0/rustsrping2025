fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 150; 
    let mut attempts = 0;
    let mut guess = 80;
    while guess != secret {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The secret number is {}.", secret);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
            guess -= 2;
        } else {
            println!("{} is too low!", guess);
            guess += 5;
        }
    }
    
    println!("It took {} attempts to guess the number.", attempts);
}
