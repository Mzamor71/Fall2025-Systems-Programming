fn variable_assignment_immutability(){
    let msg = "Hello World";
    let msg1 = "Hello World".to_string();
    let num:i32 = 10;
    let num = 8;
    let num1:i16 = 10;
    println!("{} : {}", msg, msg1);
    println!("{} := {}", num, num1);

    let mut num:i32 = 100;
    num += 100;

    println!("{}", num);
}

fn basic_control_flow(){
    let status = 500;
    if status == 200{
        println!("Status code: OK");
    }
    else if status == 500{
        println!("Status code: Internal Server Error");
    }
    else{
        println!("Page not found");
    }
}

fn concept_of_shadowing(){
    let num = 10;
    let num = num + 10;

    let num = 10;
    {
        let num = num + 10;
        println!("{}", num);
    }
    println!("{}", num);
}

fn concept_of_shadowing_mixed_up(){
    fn get_score() -> i32{return 100;}
    let result = get_score();
    print!("Student scored: {}. ", result);
    let result = result > 90;
    println!("IsPassed: {}", result);
}

fn everything_is_expression(){
    fn get_num(x:i32) -> & 'static str{
        if x == 25{
            return "Quarter";
        }
        else if x == 10{
            return "Dime";
        }
        else if x == 5{
            return "Nickel";
        }
        else{
            return "Penny";
        }
    }

    let coin_type = get_num(10);
    println!("{}", coin_type);

    let coin_type = {
        let x = 5;
        if x == 25{
            "Quarter"
        } else if x == 10{
            "Dime"
        } else if x == 5{
            "Nickel"
        } else{
            "Penny"
        }
    };
    println!("{}", coin_type);
}

fn infinite_loop(){
    let mut x = 0;
    loop { // alternative while true
        println!("{}",x);
        x +=1;

        if x == 5{ 
            break;
        }
    }
    println!("{}",x);
}

fn regular_while(){
    let mut x = 0;
    while x < 5{
        println!("{}", x);
        x += 1;
    }
}

fn for_loops_world(){
    for i in 1..5{
        if i == 3{
            continue;
        }
        println!("{}", i);
    }

    for i in (1..=5).rev(){
        println!("{}", i);
    }

    let nums = vec![1, 2, 3, 4, 5];
    let mut total = 0;
    for i in 0..5{
        println!("{}", nums[i]);
        total += i as i32;
    }
    println!("{}", total);
}

fn pattern_match_simple(){
    let num = 3;
    let letter = match num{
        1 => 'A', 2 => 'B', 3 =>{
            (64 + 1 + 2 as u8) as char
        },
        _ => '#', //rust will no guess
    };
    println!("{}", letter);
}

fn unit_function(){
    fn greet(word :&str) -> () {
        println!("Hello, {}", word)
    }
    greet("John");
}

fn return_function(){
    fn compose(word1: &str, word2: &str) -> String{
        format!("{} {}", word1, word2)
    }
    let concat = compose("Hello", "World");
    println!("{}", concat);
}

fn main() {
    // Problem 1: Let Result = num / num1
    variable_assignment_immutability();
    basic_control_flow();
    concept_of_shadowing();
    concept_of_shadowing_mixed_up();
    everything_is_expression();
    infinite_loop();
    regular_while();
    for_loops_world();
    pattern_match_simple();
    unit_function();
    return_function();
}
