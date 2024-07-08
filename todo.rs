fn get_list_items(vector: Vec<&str>) {
    for i in &vector{
        println!("{i}");
    }
}

fn main() {

// what needs have you:
	//- add an item
	//- check off an item
		//- optional; get items done that day
	//- delete an item
	//- get list of items outstanding
    // show list of actions

    let mut vec = Vec::new();

    vec.push("hi there");
    vec.push("good ol boi");

    get_list_items(vec);
}
