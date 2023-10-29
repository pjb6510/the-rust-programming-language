use std::collections::HashMap;
use std::io;

pub fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g., 'Add Sally to Engineering', 'List Engineering', 'List All', 'Quit'):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.to_lowercase() == "quit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let tokens_slice = tokens.as_slice();

        println!("slice {:?}", tokens_slice);

        match tokens_slice {
            ["Add", name, "to", department] => {
                departments
                    .entry(department.to_string())
                    .or_insert(Vec::new())
                    .push(name.to_string());
            }
            ["List", "All"] => {
                list_all(&departments);
            }
            ["List", department] => {
                list_department(&departments, department);
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
}

fn list_all(departments: &HashMap<String, Vec<String>>) {
    for (department, employees) in departments {
        println!("Department: {}", department);
        for employee in employees {
            println!("  - {}", employee);
        }
    }
}

fn list_department(departments: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(employees) = departments.get(department) {
        println!("Department: {}", department);
        for employee in employees {
            println!("  - {}", employee);
        }
    } else {
        println!("Department '{}' not found.", department);
    }
}
