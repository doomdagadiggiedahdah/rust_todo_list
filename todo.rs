fn get_list_items(vec: Vec<&str>) {
    for i in &vec{
        println!("{i}");
    }
}

fn add_item_to_list(mut vec: Vec<&str>) -> Vec<&str> {
    println!("");
    vec.push("ADDED BOI");
    vec.push("Heyyyyy made it to number two");
    vec
}

fn main() {

// what needs have you:
	//- DONE --- add an item
	//- check off an item
		//- optional; get items done that day
	//- delete an item
	//- DONE --- get list of items outstanding
    // show list of actions

    let mut vec = Vec::new();

    vec.push("hi there");
    vec.push("good ol boi");

    get_list_items(vec.clone());
    let vec = add_item_to_list(vec);
    get_list_items(vec.clone());
}
