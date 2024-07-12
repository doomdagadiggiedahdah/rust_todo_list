## motivation and kudos
I'd be lost without text and so this project won't start (or at least finish) without a 
thought dump, my old friend.

Motivation behind this whole project was from a challenge from a friend of mine, Zach Birenbaum 
for me to see how hard Rust actually is. (Some background, the only Rust I've done was using
ChatGPT to make a translucent BASIC text editor, of which I have almost no recollection. Outside
of the YouTube videos and random mentioning of something the language has, I know almost nothing.)

Further motivation for the implementation comes from my friend Thor, who ran an experiment testing
whether prototyping a program in Python before writing in Rust would save one time, and yes, it did.
Since it worked for him, a professional Rust dev, I imagine it couldn't hurt for me.
(See experiment here: https://thork.net/posts/20211015_how_effective_prototyping_euler50/)

What follows, may not have any use to you. You are warned. This will consist of me writing down
mostly when my confusions arise and walk through what's going through my head in order to get though it.
I'd love to know if this types of doc is useful, and if so, message me on github.

## criteria
- Using ChatGPT for this won't help me learn. I will be using google / SO for learning how to do this all.

## thoughts
(ooh. I could take these thoughts and turn it into a blog post.)
(as in, I have the structure of how I went about this written out, I can get rid of the irrelevant, summarize, then have a small post.)

## All tasks
- write out rest of the tasks
- interactive: ask for user input, then quit
- add to list
- delete from list
- show commands
- clear screen >:)

## Done tasks
- create list
- func to print list

### (2024.07.07__22.33.03) 
- I don't think I'll be able to finish this tonight, so I'll take out one or two chunks at a time.
- I started with just having a list to add to, so let's get a hello world, a list to add multiple things to (print it), then call it a night
- Updated.
- and I've got hello world. (22:41:10) great.
- ok, diff between macro and a function is using ! for the macros.
- ok, have an executable file after compiling. I've seen this with c++
- for this I don't think I need to read more about cargo. I'll come back to this.
- It looks like I'll need to use a vector (instead of array).
- awesome, I have a list I can add things to, and then print them out. (22:50:02) 
- What would be the next things?
    - 
- let's test the function calling (https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
    - need to figure out how to accept a vector for the function
    - yeah getting stuck on this. brb (23:10:01) -- (23:19:18) 
    - ok, figured it out. (23:28:35). I was able to use the compiler hints to figure it out
        - vec.push to add to the vector, gives the vector a type Vec<$str>, so I needed to set the type hints to accept that. And I sleep now.
- I ended up finding this guide about primitive types, macros, modules; probably helpful in the future (https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/primitive.char.html)

### (2024.07.08__19.44.03) 
- gonna try adding an item with no user interaction. print before, add, print
- yeah passing vectors is a thing here. I was curious about this.
- learning about ownership. Neat. and very relevant
    - "note: consider changing this parameter type in function `get_list_items` to borrow instead if owning the value isn't necessary". This is exactly what I want for get_list_items, let's figure that out.
- Jesus (20:27:56) added another value to the vector lol.
    - but in the process learned a lil about ownership (copy traits, there's another one).
    - reading the error messages is suprisingly helpful, very grateful.
    - I think I'll deal with this again when I get a variable length input message.
        - (but I do enjoy this "taking things slow" work. This is nice.)
- (20:55:34)  back to it. What next? let's delete something. (21:04:17) got it.
- Awesome, so I've got the basic, static functionality. But nothing that someone could interact with.
- Let's next figure out a single interactable element.
- (21:21:32) Awesome, now I've got basic interactivity. Now I need to make a case statement.
- (21:44:58) Checking in, I'm using a match statement to check which command to run and am having difficulty with the diff between String and &str. I've got a basic concept of what's going on here, so I don't think it'll be too hard.
- And now learning the diff between &String and &str. I think normally this would annoy me, but for some reason this is kind of nice.
- (22:08:18) I spoke too soon, I'm tired. Taking a break.
- When I get back, need to figure out how to get the match statement working.

### (2024.07.09) 
- (09:03:47) had an idea, got it to work. removed the interactions needing to operate on the vector, and now I've got basic interaction working. Just need to find out how to handle the vector correctly.
- but having a sign of something else not working. having the returned vector exist "somewhere". Getting a compiler error of the vec from the "add" func. This needs to be owned by something else? Will check later. (09:11:50) 

### (2024.07.12) 
- (01:21:20) I only have a bit in me. But enough to figure out what my problem is.
- Borrowing. (side note, found I can pass the references to get_todo_items. Was using .clone(), but this is less memory and "more idiomatic" source: https://stackoverflow.com/questions/42954008/how-to-pass-one-vec-to-multiple-functions-in-rust. This answer was super informative.)
- ok, some easy reading and I think I've got something (https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html). Need to make the vec mutable, and then have some mutable references.
    - oh you know it's already mutable vec
    - ahhhh this may be a deadend, can't have multiple mutable references. I'd need another for delete_item.
        - oh maybe not, can't borrow more than one at a time. Maybe.
- you know, getting tired. This was fun though. (02:12:05) 