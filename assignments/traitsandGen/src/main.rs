/*
   Name: Michael Zamora
   SID: 20593343
   Date: 11-12-2025
*/

// Define a trait representing shared student behavior
trait ShowInfo {
    fn show_info(&self);
}

// Define a struct for Undergraduate students
struct Undergrad {
    name: String,
    major: String,
    gpa: f32,
}

// Define a struct for Graduate students
struct Grad {
    name: String,
    major: String,
    gpa: f32,
    thesis: String,
}

// Implement the ShowInfo trait for Undergrad
impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad Student: {}\nMajor: {}\nGPA: {:.2}\n",
            self.name, self.major, self.gpa
        );
    }
}

// Implement the ShowInfo trait for Grad
impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Graduate Student: {}\nMajor: {}\nGPA: {:.2}\nThesis: {}\n",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}

// Enrollment struct stores *any type* implementing ShowInfo
struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Self { students: Vec::new() }
    }

    fn enroll(&mut self, student: T) {
        self.students.push(student);
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

// Generic container for mixed student types using trait objects
struct MixedEnrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

impl MixedEnrollment {
    fn new() -> Self {
        Self { students: Vec::new() }
    }

    fn enroll<T: ShowInfo + 'static>(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

fn main() {
    let u1 = Undergrad {
        name: "Alice".into(),
        major: "Computer Science".into(),
        gpa: 3.7,
    };

    let g1 = Grad {
        name: "Bob".into(),
        major: "Physics".into(),
        gpa: 3.9,
        thesis: "Quantum Field Analysis".into(),
    };

    // Example 1: Enrollment for only undergrads
    let mut undergrad_enrollment = Enrollment::new();
    undergrad_enrollment.enroll(u1);
    undergrad_enrollment.show_all();

    // Example 2: Mixed Enrollment with both grads and undergrads
    let mut mixed = MixedEnrollment::new();
    mixed.enroll(Grad {
        name: "Emma".into(),
        major: "Chemistry".into(),
        gpa: 3.8,
        thesis: "Catalyst Reaction Models".into(),
    });
    mixed.enroll(Undergrad {
        name: "Daniel".into(),
        major: "Biology".into(),
        gpa: 3.5,
    });
    mixed.show_all();
}
