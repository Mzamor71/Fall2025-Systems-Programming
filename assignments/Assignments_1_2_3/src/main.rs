/*
    Assignment 1: Temperature Converter
    Name: Michael Zamora
    SID: 20593343
*/

// C = (F - 32) * 5/9
fn fahrenheit_to_celcius(f: f64) -> f64{
    let mut c: f64 = 0.0;
    c = (f - 32.0) * 5.0/9.0;
    return c.trunc();
}

// F = (C * 9/5)+32
fn celcius_to_fahrenheit(c: f64) -> f64{
    let mut f: f64 = 0.0;
    f = (c * 9.0/5.0) + 32.0;
    return f.trunc();
}

fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true;
    }
    else{
        return false;
    }
}

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
    let mut sum: i32 = 0;
    let mut count_arr  = 0;
    let mut num_arr = [4,8,64,51,105,81];
    let mut temperature_f: f64 = 98.4815;
    let count: f64 = temperature_f + 5.0;
    println!("fahrenheit_to_celcius: {:?}C
    ", fahrenheit_to_celcius(64.0));
    println!("celcius_to_fahrenheit: {:?}F", celcius_to_fahrenheit(17.78));

    while temperature_f <= count{
        println!("temp: {}", temperature_f);
        println!("fahrenheit_to_celcius: {:?}F", fahrenheit_to_celcius(temperature_f));
        temperature_f += 1.0;
    }
    for i in 0..num_arr.len(){
        if is_even(num_arr[i]) == true{
            println!("{} is Even", num_arr[i]);
        }
        else{
            println!("{} is Odd", num_arr[i]);
        }
    }

    for i in 0..num_arr.len(){
        if num_arr[i] % 3 == 0 && num_arr[i] % 5 == 0{
            println!("FizzBuzz");
        }
        else if num_arr[i] % 3 == 0{
            println!("Fizz");
        }
        else if num_arr[i] % 5 == 0{
            println!("Buzz");
        }
        else{
            println!("{}", num_arr[i]);
        }
    }

    while true{
        if count_arr == num_arr.len(){
            println!("Sum: {}", sum);
            break;
        }
        sum += num_arr[count_arr];
        count_arr += 1;
    }

    for i in 0..num_arr.len(){
        if num_arr[i] == num_arr[0]{
            continue;
        }
        if num_arr[i] > num_arr[0]{
            num_arr[0] = num_arr[i];
        }
    }
    println!("Largest number: {}", num_arr[0]);

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