use std::io; 


fn get_todo_items(vec: &Vec<&str>) {
    for i in vec{
        println!("{i}");
    }
}

fn add_item_to_list(vec: &mut Vec<&str>) -> Vec<&str> {
    println!("");
    vec.push("ADDED BOI");
    vec.push("Heyyyyy made it to number two");
    println!("added entries");
    vec
}

fn delete_item(mut vec: Vec<&str>) -> Vec<&str> {
    let index = 2;

    println!("Removed item 2");
    vec.remove(index);


    //if (index < vec.len()) {
        //vec.remove(index)
    //} else {
        //println!("Not there!")
    //};

    vec
}

fn show_commands() {
    let list_options = vec!["Add to-do item", "Delete to-do item", "Show all to-dos", "Show all commands"];
    for i in &list_options{
        println!("{i}")
    };
}

fn main() {

// what needs have you:
    //- user interaction in all of this.
	//- DONE --- add an item
	//- DONE --- delete an item
	//- DONE --- get list of items outstanding
    //- DONE --- show list of actions

    let mut vec = Vec::new();

    vec.push("hi there");
    vec.push("good ol boi");

    //get_list_items(vec.clone());
    //let vec = add_item_to_list(vec);
    //let vec = delete_item(vec);

    println!("Welcome to the To-Do List!");
    println!("Select your option:");
    println!("Or enter 'x' to exit");
    show_commands();

    let mut user_input = String::new();

    while &*user_input.trim() != "x" {
        println!("\nSelect your option:");
        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim_end() {
            "add" => {
                let vec = add_item_to_list(&mut vec);
            }
            //"del" => {
                //let vec = delete_item(vec.clone());
            //}
            "todo" => get_todo_items(&vec),
            "comm" => show_commands(),
            _ => println!("Try again!")
        }
    }

    println!("Thanks!")

}
