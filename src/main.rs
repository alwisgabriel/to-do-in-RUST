use std::io;
use std::io::Write;

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n=== TO DO LIST ===");
        println!("1 - Add task");
        println!("2 - List tasks");
        println!("3 - Mark task as done");
        println!("4 - Remove task");
        println!("5 - Exit");

        println!("Number of tasks: {}", tasks.len());

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();

        match escolha.trim() {
            "1" => {
                print!("write task: ");
                io::stdout().flush().unwrap();

                let mut titulo = String::new();
                io::stdin().read_line(&mut titulo).unwrap();

                tasks.push(Task {
                    title: titulo.trim().to_string(),
                    done: false,
                });

                println!("task add.");
            }

            "2" => {
                println!("\ntasks:");

                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[X]" } else { "[ ]" };

                    println!("{} {} {}", i, status, task.title);
                }
            }

            "3" => {
                print!("number of the tasks: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let i: usize = index.trim().parse().unwrap();

                if i < tasks.len() {
                    tasks[i].done = true;
                    println!("fine.");
                }
            }

            "4" => {
                print!("number of the tasks: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let i: usize = index.trim().parse().unwrap();

                if i < tasks.len() {
                    tasks.remove(i);
                    println!("removed.");
                }
            }

            "5" => {
                println!("Exit...");
                break;
            }

            _ => {
                println!("option invalid.");
            }
        }
    }
}
