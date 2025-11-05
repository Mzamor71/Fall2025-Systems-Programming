


// This function takes a reference to a String as a parameter
// This means it borrows the String instead of taking ownership of it
fn my_print(word: &String){
    println!("{}", word);
}

fn update(word: &mut String){
    word.push_str("RGV");
}

fn borrow_ref_to_values(){
    let x =5;
    let y = &x;
    println!("{}", y);
    // :p formats the pointer address. The &x gets the address of x
    // So we are printing the address of y and the address of x
    println!("{:p} == {:p}", y, &x);
}

fn borrowing_doesnot_move_ownership(){
    let word = "UTRGV".to_string();
    let borrow_word =  &word;
    println!("{} == {}", word, borrow_word);
}

fn if_no_update_borrow_to_read_best_option(){
    fn my_print(word: &String){
        println!("{}", word);
    }
    let word = "UTRGV".to_string();
    my_print(&word);
}

fn borrow_to_mut_watchout(){
    let mut word = "UT".to_string();
    fn update(word: &mut String){
        word.push_str("TER DISAPPOINTMENT");
    }
    update(&mut word);
    // {word} is shorthand for {word:}
    // It prints the value of word
    println!("{word}");
}

fn explicit_deref_to_obtain_value(){
    let x: i32 = 5;
    let y = &x;
    // Dereferencing y to get the value of x
    // &i32 indicates that z is a reference to an i32
    let z: &i32 = y;
    println!("{}", z);
    // Dereferencing y to get the value of x
    // Here, *y gives us the value that y is pointing to
    let z: i32 = *y;
    println!("{}", z);
}

fn reference_has_limited_life() {
    let word_reference: &String;
    {
        let word: String = "UTRGV".to_string();
        word_reference = &word;
        // `word` goes out of scope here, and `word_reference` can no longer be used safely
    }
    // println!("{}", word_reference); // This would be unsafe and is prevented by Rust's compiler
}

fn calculate_length(s: &String) -> isize{
    // as isze converts usize to isize
    // usize is the type returned by len() method
    s.len() as isize
}

fn concat_strings(s1: &String, s2: &String) -> String{
    let mut final_word = String::new();
    final_word.push_str(s1);
    final_word.push_str(s2);
    
    final_word.to_string()
}


fn main(){
    //Section: Borrowing and References
    let word = "Discombobulate".to_string();
    //let's create a reference to word. borrow_word is a reference to word
    let borrow_word = &word;
    //Now we can use borrow_word to access the value of word without taking ownership of it
    println!("{} == {}" , word, borrow_word);

    let mut word = "UTRGV".to_string();
    // In this scope we are passing a reference to word to the function my_print. Why?
    // Because we want to use the value of word without taking ownership of it.
    // This way, after the function call, we can still use word in main.
    my_print(&word);

    update(&mut word);
    println!("Updated word: {}", word);

    borrow_ref_to_values();

    borrowing_doesnot_move_ownership();

    if_no_update_borrow_to_read_best_option();

    borrow_to_mut_watchout();

    explicit_deref_to_obtain_value();

    reference_has_limited_life();

    let string_1 = String::from("Watermelon Butterfly");
    let length = calculate_length(&string_1);
    println!("The length of '{}' is {}.", string_1, length);

    let string_1_1 = String::from("Where Are You Going? ");
    let string_2_2 = String::from("I am Coming Home.");
    let concat_strings_result = concat_strings(&string_1_1, &string_2_2); 

    println!("Concatenated String: {}", concat_strings_result);


}