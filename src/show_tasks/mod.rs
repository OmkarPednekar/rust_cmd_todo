pub fn show(tasks_arr: &Vec<String>) {
    for (x, i) in tasks_arr.iter().enumerate() {
        println!("{x}-{i}");
    }
}
