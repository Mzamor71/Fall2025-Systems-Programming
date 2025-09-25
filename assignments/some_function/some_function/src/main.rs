/*
    Name: Michael Joseph Zamora
    Student ID: 20593343
    Date: 09/24/2025
*/

fn concat_strings(s1: &String, s2: &String) -> String{
    // Concatenate two strings and return the result
    let mut result = s1.clone(); // Clone s1 to avoid moving it
    result.push_str(s2); // Append s2 to the cloned string
    result // Return the concatenated
}

fn clone_and_modify(s: &String) -> String{
    // Clone the input string, modify the clone, and return it
    let mut cloned = s.clone(); // Clone the input string
    cloned.push_str("World!"); // Modify the cloned string
    cloned // Return the modified clone
}

fn sum(total: &mut i32, low: i32, high: i32){
    // Calculate the sum of integers from low to high (inclusive) and update total
    *total = (low..=high).sum(); // Calculate the sum from low to high and update total
}

fn main(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    let mut total = 0;
    sum(&mut total, 1, 5);
    println!("Sum from 1 to 5 is: {}", total); // Should print: "Sum from 1 to 5 is: 15"
}