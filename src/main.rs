use std::io;

fn main() -> ! {
    loop {
        println!();
        println!("Welcome to Jamii Company, choose an option you would like to perform");
        println!("1. Add employee name to department");
        println!("2. Display all employee names in the company");
        println!("3. Display all employee names according to department");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please pick the right choice!!!");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Choose Department");
                println!("1. Engineering");
                println!("2. Sales");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("failed to read line");

                let department: u32 = match department.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please pick the right choice!!!");
                        continue;
                    }
                };

                match department {
                    1 => println!("Engineering department"),
                    2 => println!("Sales department"),
                    _ => println!("Please pick the right choice!!!"),
                }
            }
            2 => println!("You have picked choice 2"),
            3 => {
                println!("Choose Department");
                println!("1. Engineering");
                println!("2. Sales");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("failed to read line");

                let department: u32 = match department.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please pick the right choice!!!");
                        continue;
                    }
                };

                match department {
                    1 => println!("Engineering department"),
                    2 => println!("Sales department"),
                    _ => println!("Please pick the right choice!!!"),
                }
            }
            _ => println!("Invalid choice! Please choose between 1, 2 or 3"),
        }
    }
}
