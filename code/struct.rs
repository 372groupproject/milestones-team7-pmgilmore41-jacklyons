struct Student {
    First: String,
    Last: String,
    Grade: f32,
    ID: i32,
}

fn main(){
    let mut John = Student {
        First: String::from("John"),
        Last: String:: from("Deer"),
        Grade: 86.7,
        ID: 112554,
    };

    println!("ID {}, NAME {} {}, GRADE {}, ",John.ID, John.First, John.Last, John.Grade);
}