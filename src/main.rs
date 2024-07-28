use cmd_todo::{ add_task, mark_complete, remove_task, show_tasks };
use std::{ io };
fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let mut user_name: String = String::new();
    let mut action: String = String::new();
    println!("Welcome User Please Enter your Name");
    io::stdin().read_line(&mut user_name).expect("Enter a valid Name");
    println!("Hi {},Please Select any one of the following actions", user_name);
    loop {
        println!(
            " 1. Add a Task \n 2. Complete a Task \n 3.Remove a Task \n 4.Show All Tasks\n5.Exit"
        );
        match io::stdin().read_line(&mut action) {
            Ok(_) => {
                match action.trim().parse::<u32>() {
                    Ok(number) => {
                        action = number.to_string();
                    }
                    Err(_) => println!("Failed to parse a number."),
                }
            }
            Err(error) => println!("Error reading line: {}", error),
        }
        let res = action.trim().parse::<u32>();
        match res {
            Ok(1) => {
                add_task::task_adder(&mut tasks);
            }
            Ok(2) => {
                mark_complete::complete(&mut tasks);
            }
            Ok(4) => {
                show_tasks::show(&tasks);
            }
            Ok(5) => {
                break;
            }
            _ => println!("DEFAULT BEHAVIOR"),
        }
        action.clear();
    }
}
