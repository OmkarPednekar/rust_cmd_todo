use std::io;
pub fn task_adder(tasks_vec: &mut Vec<String>) {
    let mut task: String = String::new();
    println!("Enter the task you want to add");
    io::stdin().read_line(&mut task).expect("ENTER VALID A TASK");
    tasks_vec.push(task);
}
