/*
    Assignment 2: Number Analyzer
    Name: Michael Zamora
    SID: 20593343
*/

fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true;
    }
    else{
        return false;
    }
}

fn main(){
    let mut sum: i32 = 0;
    let mut count  = 0;
    let mut num_arr = [4,8,64,51,105,81,9100,55,34,37,5000];

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
        if count == num_arr.len(){
            println!("Sum: {}", sum);
            break;
        }
        sum += num_arr[count];
        count += 1;
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
}