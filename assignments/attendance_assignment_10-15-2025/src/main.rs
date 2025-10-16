/*
    Name:       Michael Zamora
    SID:        20593343
    Date:       10-15-2025
    Assignment: Attendance Assignment 10-15-2025
*/

// Define an enum to represent different types of fruits
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}
// Define a struct to represent an inventory of fruits
struct Inventory {
    fruit: Vec<Fruit>,
}
// Implement methods for the Inventory struct
impl Inventory {
    fn available_fruits(&self) {
        for fruit in &self.fruit {
            Self::tell_me_joke(fruit);
        }

    }

    fn tell_me_joke(fruit: &Fruit) { 
        match fruit {
            Fruit::Apple(joke) => println!("Apple Joke: {}", joke),
            Fruit::Banana(joke) => println!("Banana Joke: {}", joke),
            Fruit::Tomato(joke) => println!("Tomato Joke: {}", joke),
        }

    }
}
// Main function to demonstrate the usage of the Inventory and Fruit enum
fn main(){
    let a = "Why did the apple stop in the middle of the road? It ran out of juice!".to_string();
    let b = "Why did the banana go to the doctor? It wasn’t peeling well!".to_string();
    let t = "Why did the tomato turn red? Because it saw the salad dressing!".to_string();
    let fruits = vec![
        Fruit::Banana(b),
        Fruit::Apple(a),
        Fruit::Tomato(t)
        ];
    let grocery_store = Inventory {
        fruit:fruits,
    };

    grocery_store.available_fruits();
}