use std::io;
use ansi_term::Colour::Green;
pub fn complete(tasks_arr: &mut Vec<String>) {
    let mut index: String = String::new();

    println!("Select the index of the task that you want to complete");
    for (i, x) in tasks_arr.iter().enumerate() {
        println!("{i}-{x}");
    }
    io::stdin().read_line(&mut index).expect("Error");
    let trim_value: Result<u32, std::num::ParseIntError> = index.trim().parse::<u32>();
    match trim_value {
        Ok(trim_value) if trim_value > (tasks_arr.len() as u32) => {
            println!("INDEX NOT FOUND");
        }
        Ok(trim_value) if trim_value < (tasks_arr.len() as u32) => {
            let completed_text: ansi_term::ANSIGenericString<str> = Green.bold()
                .strikethrough()
                .paint(&tasks_arr[trim_value as usize]);
            tasks_arr[trim_value as usize] = completed_text.to_string();
        }
        _ => println!("DEFAULT"),
    }
    for (i, x) in tasks_arr.iter().enumerate() {
        println!("{i}-{x}");
    }
}
