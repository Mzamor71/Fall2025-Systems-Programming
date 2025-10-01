/*
    Name: Michael Zamora
    SID: 20593343
    Date: 10/01/2025
*/
//Assignment 1: Student Struct
struct Student{
    name: String,
    major: String
}

impl Student{
    fn new() -> Student{
        Student{
            name: String::from(""),
            major: String::from("")
        }
    }
    fn get_major(&self) -> &String{
        &self.major
    }
    fn set_major(&mut self, major: String){
        self.major = major;
    }
    fn set_name(&mut self, name: String){
        self.name = name;
    }
    fn get_name(&self) -> &String{
        &self.name
    }
}


fn main(){

    let mut student1 = Student::new();
    student1.set_major(String::from("Computer Science"));
    println!("Student major: {}", student1.get_major());
    student1.set_name(String::from("Gertrude"));
    println!("Student name: {}", student1.get_name());


}