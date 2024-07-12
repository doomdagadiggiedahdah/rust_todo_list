## make a cli based todolist (2024.07.03__23.23.20) -- (2024.07.04__00.04.42) (have basic functionality. )

import os

list_of_items = []
list_of_items = ["apple"]

def add_item_to_list():
    new_item = input("enter task to do:\n")
    list_of_items.append(new_item)
    return print(f"'{new_item}' was added to list. Nice")

def delete_item():
    enum_list_items = { ind:item for (ind,item) in enumerate(get_todo_items()) } 
    print(enum_list_items)

    ## could use some error handling, what if it's not in the list?
    delete_item = int(input("which item do you want to delete?\n"))
    del list_of_items[delete_item]
    return print(f"{delete_item} was removed from the list")

    ## this one has something here. need to associate a number to an option so they have something to pick from.
    ## AND have that be scalable to multiple items. I can enumerate, but how do I maintain that.
    ## store the enum and item in a dictionary, take the enum as input and find that item in dictionary, remove from list
    #myDict = { k:v for (k,v) in zip(keys, values)}  
    ## then I can prove it by displaying list

def get_todo_items():
    return list_of_items

clear_screen = lambda: os.system("clear")
    
def show_commands():
    list_options = ["Add item", "Delete item", "Show all items", "Show help"]
    ## It'd be nice to have them spaced out on multiple lines, for now this'll work
    return print(list_options)

print("todo list started. Enter command: ")
print("1. Add item 2. Delete item 3. Show items 4. Clear screen 5. Show commands 6. Exit (enter number)") 
## I could do the same dict comp as above for this list of options
## and then if I can print docstrings, give a short description as well

while True:
    ## need to get user input
    action = input()
    if action == "1":
        add_item_to_list()
    if action == "2":
        delete_item()
    if action == "3":
        print(get_todo_items())
    if action == "4":
        clear_screen()
    if action == "5":
        show_commands()
    if action.lower() == "6" or "exit":
        print("goodbye!")
        break

    print("continue to next task")
    continue

## so, how hard do I want to make this?
## I think this is enough for the purpose of this exercise.
## I can make more difficult with the Rust, and then when I want an easier version of it, 
# I implement over here first to make sure the thought is fleshed out.



## what needs have you:
	##- add an item
	##- check off an item
		##- optional; get items done that day
	##- delete an item
	##- get list of items outstanding
    ## show list of actions

## Add item
# needs a container to push to (I wonder how Rust treats variable size objects)
# need a command to add to it
# prompt using saying "{task} was added"

## check off on item
# currently, this just deletes the item.
# access the list of todos, pick number to remove
# prompt using saying "{task} was deleted"

## get list of outstanding items
# print out list of items (this will be first).
# I want this to eventually save to a file so the items are persistent when you close.

## show avail actions
# I know there's a way to list all actions and a description of what they do

## clear screen when next command is run.

