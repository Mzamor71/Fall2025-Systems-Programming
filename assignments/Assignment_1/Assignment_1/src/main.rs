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

fn main(){
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
}
