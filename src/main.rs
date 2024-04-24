use std::{collections::HashMap, io};

// Function to get user input as u32
fn get_user_input(message: &str) -> u32 {
    loop {
        println!("{}", message);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number\n"),
        }
    }
}

// Function to add employee to department
fn add_employee_to_department(company: &mut HashMap<String, String>) {
    let department_message = "Choose Department\n1. Engineering\n2. Sales";
    let department_choice = get_user_input(department_message);

    let department = match department_choice {
        1 => "Engineering",
        2 => "Sales",
        _ => {
            println!("Please pick the right choice!!!");
            return;
        }
    };

    loop {
        println!("Enter employee name. Type 'exit' to return to main menu:");
        let mut employee_name = String::new();
        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");

        let employee_name = employee_name.trim();

        if employee_name.is_empty() {
            println!("Employee name cannot be empty!!");
            return;
        }

        if !employee_name.chars().all(char::is_alphabetic) {
            println!("Employee name must contain only alphabetic characters!");
            return;
        }

        if employee_name.eq_ignore_ascii_case("exit") {
            println!("Returning to main menu");
            break;
        } else {
            company.insert(employee_name.to_string(), department.to_string());
            println!(
                "Employee {} added to {} department.\n",
                employee_name.trim(),
                department
            );
        }
    }
}

// Function to display employee names according to department
fn display_employee_names_by_department(company: &HashMap<String, String>) {
    let department_message = "Choose Department\n1. Engineering\n2. Sales";
    let department_choice = get_user_input(department_message);

    let department = match department_choice {
        1 => "Engineering",
        2 => "Sales",
        _ => {
            println!("Please pick the right choice!!!");
            return;
        }
    };

    let mut department_employees: Vec<_> = company
        .iter()
        .filter(|(_, dep)| dep == &&department)
        .map(|(name, _)| name)
        .collect();

    department_employees.sort();

    if department_employees.is_empty() {
        println!("No employees in {} department.", department);
    } else {
        println!("Employees in {} department:", department);
        for employee in department_employees {
            println!("{}", employee);
        }
    }
}

// function to display all employee names in Jamii Company
fn display_all_employee_names(company: &HashMap<String, String>) {
    if company.is_empty() {
        println!("No employees in the company.");
    } else {
        let mut sorted_employees: Vec<_> = company.iter().map(|(name, _)| name).collect();
        sorted_employees.sort();
        println!("Employees in the company:");
        for employee in sorted_employees {
            println!("{} - {}", employee, company.get(employee).unwrap());
        }
    }
}

fn main() {
    let mut company: HashMap<String, String> = HashMap::new();
    loop {
        println!();
        println!("Welcome to Jamii Company, choose an option you would like to perform");
        println!("1. Add employee name to department");
        println!("2. Display all employee names in the company");
        println!("3. Display all employee names according to department");
        println!("4. Exit program");

        let choice = get_user_input("Enter your choice:");

        match choice {
            1 => add_employee_to_department(&mut company),
            2 => display_all_employee_names(&company),
            3 => display_employee_names_by_department(&company),
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice! Please choose between 1, 2, 3 or 4"),
        }
    }
}
