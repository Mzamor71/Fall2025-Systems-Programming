#[derive(Debug)]
enum PaymentMethod{
    Cash,
    CreditCard,
    DebitCard,
}

#[derive(Debug)]
enum Genre {
    Fiction,
    NonFiction,
    Mystery,
    ScienceFiction,
}

struct Book{
    title: String,
    author: String,
    genre: Genre,
    publication_year: u16,
}

#[derive(PartialEq, Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}



fn main() {
    let cash = PaymentMethod::Cash;
    let credit = PaymentMethod::CreditCard;
    let debit = PaymentMethod::DebitCard;

    println!("Payment method selected: {:?}", cash);
    println!("Payment method selected: {:?}", credit);
    println!("Payment method selected: {:?}", debit);

    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        genre: Genre::NonFiction,
        publication_year: 2018,
    };

    println!("Book: {} by {}", book.title, book.author);
    println!("Genre: {:?}", book.genre);
    println!("Published: {}", book.publication_year);

    let light = TrafficLight::Green;

    if light == TrafficLight::Green {
        println!("Go!");
    } else if light == TrafficLight::Yellow {
        println!("Prepare to stop.");
    } else {
        println!("Stop!");
    }

    // Using a match statement for more concise code
    match light {
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow => println!("Prepare to stop."),
        TrafficLight::Red => println!("Stop!"),
    }
    
}
