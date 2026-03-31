struct Employee {
    name: String,
    salary: u32,
}

struct EmployeeRecord {
    employee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecord {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.employee_db.is_empty() {
            let employee = self.employee_db.remove(0);
            Some(employee.name)
        } else {
            None
        }
    }
}

fn main() {
    let emp1 = Employee {
        name: String::from("Adil"),
        salary: 2000,
    };

    let emp2 = Employee {
        name: String::from("Adnan"),
        salary: 32000,
    };

    let mut empl_db = EmployeeRecord {
        employee_db: vec![emp1, emp2],
    };

    println!("{:?}", empl_db.next());
    println!("{:?}", empl_db.next());
}
