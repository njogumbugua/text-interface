use std::io;

fn main() {
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

        let choice: u32 = choice.trim().parse().unwrap();

        println!("you have picked choice {}", choice);
    }
}
