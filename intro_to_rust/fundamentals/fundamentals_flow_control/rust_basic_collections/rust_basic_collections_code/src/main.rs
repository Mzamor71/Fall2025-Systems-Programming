

fn main(){
    //Arrays: fixed-size collections of elements of teh same type, stored continuously in memory
    //The i32 represents the data type being a 32 bit integer and 5 represents the amount of contents in the array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The third element is: {}", numbers[2]);
    println!("The array is: {:?}", numbers);
    println!("Array Length: {}", numbers.len());

    /*Vectors: dynamic growable arrays
    Dynamically Sized
    Stored on the heap
    Can grow or shrink at runtime
    */
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    let mut fruits = vec!["apple", "banana", "cherry"];

    println!("The second fruit is: {}", fruits[1]);

    for fruit in &fruits {
        println!("I like {}", fruit);
    }

    fruits[1] = "blueberry";

    fruits.pop();

    println!("Updated fruits: {:?}", fruits);

    /*Slices: A way to view into a contiguous sequenece of elements
    in a collection
    - Reference to contiguous sequence of elements
    - Can be mutable or immutable
    - Allows working with a subset of a collection
    */

    let numbers = [1, 2, 3, 4, 5];

    let all_numbers = &numbers[..];
    println!("All numbers: {:?}", all_numbers);

    let middle_numbers =  &numbers[1..4];
    println!("Middle numbers: {:?}", middle_numbers);

    fn sum_slice(slice: &[i32]) -> i32{
        slice.iter().sum()
    }
    println!("Sum of all numbers: {}", sum_slice(&numbers));
    println!("Sum of middle numbers: {}", sum_slice(middle_numbers));
}