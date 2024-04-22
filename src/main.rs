use std::io;

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
fn add_employee_to_department() {
    let department_message = "Choose Department\n1. Engineering\n2. Sales";
    let department_choice = get_user_input(department_message);

    match department_choice {
        1 => println!("Engineering department\nInsert employee name"),
        2 => println!("Sales department\nInsert employee name"),
        _ => println!("Please pick the right choice!!!"),
    }
}

// Function to display employee names according to department
fn display_employee_names_by_department() {
    let department_message = "Choose Department\n1. Engineering\n2. Sales";
    let department_choice = get_user_input(department_message);

    match department_choice {
        1 => println!("Engineering department\nThere are 20 employees in Engineering Department"),
        2 => println!("Sales department\nThere are 50 employees in Sales Department"),
        _ => println!("Please pick the right choice!!!"),
    }
}

// function to display all employee names in Jamii Company
fn display_all_employee_names() {
    println!("There are 100 employees at Jamii Company");
}

fn main() {
    loop {
        println!();
        println!("Welcome to Jamii Company, choose an option you would like to perform");
        println!("1. Add employee name to department");
        println!("2. Display all employee names in the company");
        println!("3. Display all employee names according to department");
        println!("4. Exit program");

        let choice = get_user_input("Enter your choice:");

        match choice {
            1 => add_employee_to_department(),
            2 => display_all_employee_names(),
            3 => display_employee_names_by_department(),
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice! Please choose between 1, 2, or 3"),
        }
    }
}
