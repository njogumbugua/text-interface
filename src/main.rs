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
            Err(_) => println!("Please enter a valid number"),
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
    println!("Enter employee name:");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");

    company.insert(employee_name.trim().to_string(), department.to_string());
    println!(
        "Employee {} added to {} department.",
        employee_name.trim(),
        department
    );
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

    let mut count = 0;
    println!("Employees in {} department:", department);
    for (employee, dep) in company {
        if dep == department {
            println!("{}", employee);
            count += 1;
        }
    }
    if count == 0 {
        println!("No employees in {} department.", department);
    }
}

// function to display all employee names in Jamii Company
fn display_all_employee_names(company: &HashMap<String, String>) {
    if company.is_empty() {
        println!("No employees in the company.");
    } else {
        println!("Employees in the company:");
        for (employee, department) in company {
            println!("{} - {}", employee, department);
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
            _ => println!("Invalid choice! Please choose between 1, 2, or 3"),
        }
    }
}
