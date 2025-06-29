pub fn main () {
    println!("Available commands:\n");
    println!("  General");
    println!("      :exit or :quit - Exit the application\n");
    println!("  Tasks");
    println!("      :tasks - List all tasks");
    println!("      :add <task name> - Add a new task");
    println!("      :delete <task_id> - Remove a task by ID");
    println!("      :rename <task_id> <new_name> - Edit a task's name");
    println!("      :complete <task_id> - Mark a task as complete");
    println!("      :reopen <task_id> - Mark a task as incomplete");
    println!("  Timers");
    println!("      :timers - List all timers");
    println!("      :start <note> - Start a new timer with a note");
    println!("      :end <timer_id> - End a timer by ID");
    println!("      :delete <timer_id> - Delete a timer by ID");
}