use std::fs::File;

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn main(){
    //Integer Data Types Signed And Unsigned

    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = 2_147_483_647;
    let d: u64 = 18_446_744_073_709_551_615;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    //Floating Point Numbers

    let x: f32 = 3.14;
    let y: f64 = 2.71828;

    println!("x: {}, y: {}", x, y);

    //Booleans

    let is_active: bool = true; //Result True;
    let is_greater = 10 > 5; //Result True;

    //Try Some More Cases and Examples:

    println!("is_active: {}, is_greater: {}", is_active, is_greater);

    //Characters

    let letter: char = 'A';
    let emoji: char = '😀';
    
    println!("letter: {}, emoji: {}", letter, emoji);

    //More Compound Types of Data
    //Arrays

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0, 3]; // [0, 0, 0]

    println!("First number: {}", numbers[0]);
    println!("Array Length: {}", zeros.len());

    //Tuples: Variables that are given multiple data types
    
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);

    println!("Name: {}, Age: {}, Employed: {}", person.0, person.1, person.2);

    //Vectors: Dynamic Arrays that can grow and shrink

    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);

    println!("Vector: {:?}", vec);
    println!("Vector Length: {}", vec.len());

    //Text Types: String and &str: An Immutable reference to a string slice
    //String:
    let mut s: String = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);

    //&str:
    let s: &str = "Hello, World!";
    
    println!("{}", s);

    //Pointer Types
    let x = 5;
    let y = &x;
    
    println!("x: {}, y: {}", x, *y);

    //Box: Box<T> is used for heap allocation

    let boxed_int: Box<i32> = Box::new(5);
    println!("Boxed Value: {}", *boxed_int);

    //Option<T>: Represents an Optional Value

    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    match some_number{
        Some(n) => println!("Got a number: {}", n),
        None => println!("No Number"),
    }

    match no_number{
        Some(n) => println!("Got a number: {}", n),
        None => println!("No Number"),
    }

    //Result Types: used for returning and propagating errors

    let file_result = File::open("nonexistent.txt");
    match file_result{
        Ok(_file) => println!("File opened successfully"),
        Err(error) => println!("Error opening file: {:?}", error),
    }

    //Function Pointers: allows you to store and pass around 
    // references to functions

    let operation: fn(i32, i32) -> i32 = add;
    let result = operation(5, 3);

    println!("Result: {}", result);
}
