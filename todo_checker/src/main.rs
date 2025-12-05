enum TaskStatus {
    ToDo,
    InProgress,
    Done,
    Cancelled,
}

struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

fn check_and_report_status(task: &Task) {
    println!("---Task ID : {}---", task.id);
    println!("title: {}", task.title);

    match task.status {
        TaskStatus::ToDo => {
            println!("Status: To Do ");
        }
        TaskStatus::InProgress => {
            println!("Keep going! YOur task is in progress.");
        }
        TaskStatus::Done => {
            println!("Congratulations!");
        }
        TaskStatus::Cancelled => {
            println!("Cancelled task.");
        }
    }
}

fn main() {
    let task1 = Task {
        id: 1,
        title: String::from("Make a plan of learning Rust"),
        status: TaskStatus::Done,
    };
    
    let task2 = Task {
        id: 2,
        title: String::from("Implement a project in Rust"),
        status: TaskStatus::InProgress,
    };
    
    let task3 = Task {
        id: 3,
        title: String::from("Review Rust documentation"),
        status: TaskStatus::ToDo,
    };
    
    let task4 = Task {
        id: 4,
        title: String::from("Old task"),
        status: TaskStatus::Cancelled,
    };

    check_and_report_status(&task1);
    check_and_report_status(&task2);
    check_and_report_status(&task3);
    check_and_report_status(&task4);
    
    println!("\nAll tasks processed.");
}