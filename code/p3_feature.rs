use std::io;
#[allow(non_snake_case)]
struct Employee {
    First: char,
    Last: char,
    ID: i32,
    Salary: f32,
}

impl Copy for Employee {}
#[allow(non_snake_case)]
impl Clone for Employee {
    fn clone(&self) -> Employee {
        Employee{First: self.First, Last: self.Last, ID: self.ID, Salary:self.Salary}
    }
}
#[allow(non_snake_case)]
impl Employee {
    fn new() -> Employee {
        Employee {First: 'D', Last: 'D', ID: -1, Salary: -1.0}
    }
}
#[allow(non_snake_case)]
fn main(){
    
    println!("Welcome to your new employee DataBase!\nYou have purchased space for 10 records.");

    let mut empdata = [Employee::new(); 10];
    let mut index = 0;
    loop {
        println!("New Record? (Y/N)");
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        let answer: char = buff.trim().parse::<char>().unwrap();
        
        if answer=='N' {
            break;
        }
        else {
            buff = String::new();
            println!("First initial?");
            io::stdin().read_line(&mut buff).unwrap();
            empdata[index].First = buff.trim().parse::<char>().unwrap();
            buff = String::new();
            println!("Last initial?");
            io::stdin().read_line(&mut buff).unwrap();
            empdata[index].Last = buff.trim().parse::<char>().unwrap();
            buff = String::new();
            println!("Employee ID?");
            io::stdin().read_line(&mut buff).unwrap();
            empdata[index].ID = buff.trim().parse::<i32>().unwrap();
            buff = String::new();
            println!("Salary?");
            io::stdin().read_line(&mut buff).unwrap();
            empdata[index].Salary = buff.trim().parse::<f32>().unwrap();
            index += 1;
        }
    }

    let mut total_payroll: f32 = 0.0;
    let mut total_active: i16 = 0;
    for elem in empdata.iter() {
        if elem.ID == -1 {
            break;
        }
        else {
            println!("EMPLOYEE INITIALS {}{}, ID {}, SALARY {}", elem.First,elem.Last,elem.ID,elem.Salary);
            total_active += 1;
            total_payroll += elem.Salary;
        }
    }

    println!("Total employees active: {}", total_active);
    println!("Total payroll: {} with average of {}",total_payroll,total_payroll/(total_active as f32));
}
