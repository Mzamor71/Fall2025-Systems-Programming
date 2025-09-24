/*
    Assignment 3: Guessing Game
    Name: Michael Zamora
    SID: 20593343
*/


fn check_guess(guess: i32, secret_number: i32) -> bool{
    if guess < secret_number{
        println!("Too Low!");
        return false;
    }
    else if guess > secret_number{
        println!("Too High!");
        return false;
    }
    else{
        println!("Guess is Correct!");
        return true;
    }
}

fn main(){
    let secret_number = 49;

    let mut guess = String::new();

    loop{
        println!("Enter your guess: ");
        // Take input from user
        guess.clear();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if check_guess(guess, secret_number) == true{
            break;
        }
    }


}