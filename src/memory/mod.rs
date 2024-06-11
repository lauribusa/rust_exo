pub fn memory_allocation(){
    let x = 5;
    let y = x; 
    // y is the new owner of 5 and x is destroyed;
    // these are stored in the stack (very fast, small storage space)

    println!("{x} {y}");
}

pub fn memory_allocation_heap(){
    let x = Box::new(5);
    let y = x;

    // Box::new() assigns the value to the heap instead of stack
    // heap is slow but has huge storage space (computer RAM)
    // dynamic values are also assigned to heap as their size can be changed
    // vectors are an example of dynamic values automatically assigned to the heap.
}

pub fn memory_allocation_next(){
    let v = vec![5];
    let y = v.clone();

    // Vectors are variables with unknown size at compile time, therefore it's stored to heap
    // to copy a value, it needs to declare .clone explicitly or it will not compile to let the program know it must duplicate values
    // it lets the developer know it's a costly operation.
}

pub fn memory_allocation_behavior(){
    let s = String::from("Péon");
    // consume_string(s); // function becomes owner of s
    // println!("{s}");  this won't work as 's' has been destroyed since the previous function became the owner of the value.
    // let s = use_string(s);
    // one method is to return the value to get ownership back
    borrow_string(&s);
    // the better solution is to borrow the value with &. That way, the value is not moved to another owner, and instead the value is passed through the function then returned to its owner when the process is done.
    println!("{s}"); // this will work as the owner is still "s".

}

fn consume_string(s: String){
    println!("{s}");
}

fn use_string(s: String) -> String {
    println!("{s}");
    s
}

fn borrow_string(s: &String){
    println!("{s}");
}

// values (owners) can be mutable
// but their references aren't.
// mutable references must be explicitly specified.

pub fn mutable_reference(){
    let mut s = String::from("Morandini");
    mutify(&mut s);

}

fn mutify(s: &mut String){
    s.push_str(" est complètement con");
}

pub fn todo_list_creation(){
    // 1. Create task
    // 2. Get tasks
    // 3. Quit
    let mut tasks: Vec<String> = vec![];
    show_commands();
    loop {
        let input = read_input();
        if input == "1" {
            println!("Please input the task:");
            println!("Enter 'exit' to quit creation mode.");
            loop {
                let task = read_input();
                if task == "exit" {
                    println!("Creation mode ended.");
                    show_commands();
                    break;
                }
                tasks.push(task);
            }
        }

        if input == "2" {
            if tasks.len() <= 0 {
                println!("No tasks to display.");
            } else {
                println!("Printing current list of tasks...");
                for task in tasks.iter() {
                    println!("[ {} ]", task);
                }
            }
            show_commands();
        }

        if input == "3" {
            println!("Goodbye.");
            break;
        }
    }
}

fn show_commands(){
    println!("___________");
    println!("List of commands:");
    println!("1. Create task");
    println!("2. Get tasks");
    println!("3. Quit");
    println!("___________");
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read");
    input.trim().to_string()
}
